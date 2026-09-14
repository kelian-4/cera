#include "terminal/Terminal.hpp"

#include <cerrno>
#include <csignal>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <string>
#include <sys/ioctl.h>
#include <unistd.h>

namespace cera::terminal {

namespace {
volatile sig_atomic_t g_resizePending = 0;

void handleSigwinch(int) {
    g_resizePending = 1;
}
}

void Terminal::installResizeHandler() {
    struct sigaction sa{};
    sa.sa_handler = handleSigwinch;
    sigemptyset(&sa.sa_mask);
    sa.sa_flags = SA_RESTART;
    sigaction(SIGWINCH, &sa, nullptr);
}

bool Terminal::consumeResizeFlag() {
    if (g_resizePending) {
        g_resizePending = 0;
        return true;
    }
    return false;
}

void die(const char* s) {
    ::write(STDOUT_FILENO, "\x1b[?1049l", 8);

    perror(s);
    std::exit(1);
}

Terminal::Terminal() {
    enableRawMode();
    ::write(STDOUT_FILENO, "\x1b[?1049h", 8);
}

Terminal::~Terminal() {
    disableMouseReporting();
    ::write(STDOUT_FILENO, "\x1b[?1049l", 8);
    disableRawMode();
}

void Terminal::enableRawMode() {
    if (tcgetattr(STDIN_FILENO, &orig_termios_) == -1) {
        die("tcgetattr");
    }
    raw_mode_enabled_ = true;

    struct termios raw = orig_termios_;
    raw.c_iflag &= ~(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    raw.c_oflag &= ~(OPOST);
    raw.c_cflag &= ~(CS8);
    raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG);
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;

    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) {
        die("tcsetattr");
    }
}

void Terminal::disableRawMode() {
    if (raw_mode_enabled_) {
        tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios_);
        raw_mode_enabled_ = false;
    }
}

void Terminal::write(const char* buf, size_t len) {
    ::write(STDOUT_FILENO, buf, len);
}

void Terminal::enableMouseReporting() {
    ::write(STDOUT_FILENO, "\x1b[?1002h\x1b[?1006h", 16);
}

void Terminal::disableMouseReporting() {
    ::write(STDOUT_FILENO, "\x1b[?1006l\x1b[?1002l", 16);
}

bool Terminal::getCursorPosition(int* rows, int* cols) {
    char buf[32];
    unsigned int i = 0;
    if (::write(STDOUT_FILENO, "\x1b[6n", 4) != 4) return false;

    while (i < sizeof(buf) - 1) {
        if (read(STDIN_FILENO, &buf[i], 1) != 1) break;
        if (buf[i] == 'R') break;
        i++;
    }
    buf[i] = '\0';

    if (buf[0] != '\x1b' || buf[1] != '[') return false;
    if (sscanf(&buf[2], "%d;%d", rows, cols) != 2) return false;
    return true;
}

bool Terminal::getWindowSize(int* rows, int* cols) {
    struct winsize ws;
    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        if (::write(STDOUT_FILENO, "\x1b[999C\x1b[999B", 12) != 12) return false;
        return getCursorPosition(rows, cols);
    }
    *cols = ws.ws_col;
    *rows = ws.ws_row;
    return true;
}

namespace {

int readByteBlocking() {
    int nread;
    char c;
    while ((nread = read(STDIN_FILENO, &c, 1)) != 1) {
        if (nread == -1 && errno != EAGAIN) die("read");
    }
    return static_cast<unsigned char>(c);
}

int sequenceLengthForLead(unsigned char lead) {
    if ((lead & 0x80) == 0x00) return 1;
    if ((lead & 0xE0) == 0xC0) return 2;
    if ((lead & 0xF0) == 0xE0) return 3;
    if ((lead & 0xF8) == 0xF0) return 4;
    return 1;
}

uint32_t decodeUtf8Sequence(unsigned char lead, int len) {
    uint32_t cp = 0;
    unsigned char cont[3] = {0, 0, 0};
    for (int k = 0; k < len - 1; k++) {
        cont[k] = static_cast<unsigned char>(readByteBlocking());
    }
    if (len == 1) {
        cp = lead;
    } else if (len == 2) {
        cp = static_cast<uint32_t>(lead & 0x1F);
        cp = (cp << 6) | (cont[0] & 0x3F);
    } else if (len == 3) {
        cp = static_cast<uint32_t>(lead & 0x0F);
        cp = (cp << 6) | (cont[0] & 0x3F);
        cp = (cp << 6) | (cont[1] & 0x3F);
    } else {
        cp = static_cast<uint32_t>(lead & 0x07);
        cp = (cp << 6) | (cont[0] & 0x3F);
        cp = (cp << 6) | (cont[1] & 0x3F);
        cp = (cp << 6) | (cont[2] & 0x3F);
    }
    return cp;
}

}

int Terminal::readKey() {
    int nread;
    char c;
    nread = read(STDIN_FILENO, &c, 1);
    if (nread == 0) {
        return NO_KEY;
    }
    if (nread == -1) {
        if (errno != EAGAIN) die("read");
        return NO_KEY;
    }

    unsigned char uc = static_cast<unsigned char>(c);

    if (uc == '\x1b') {
        char seq[3];
        if (read(STDIN_FILENO, &seq[0], 1) != 1) return '\x1b';
        if (read(STDIN_FILENO, &seq[1], 1) != 1) return '\x1b';

        if (seq[0] == '[') {
            if (seq[1] == '<') {
                int values[3] = {0, 0, 0};
                int valueIdx = 0;
                char finalCh = 'M';
                while (true) {
                    int b = readByteBlocking();
                    if (b == ';') {
                        valueIdx++;
                        if (valueIdx > 2) break;
                        continue;
                    }
                    if (b == 'M' || b == 'm') {
                        finalCh = static_cast<char>(b);
                        break;
                    }
                    if (b >= '0' && b <= '9' && valueIdx <= 2) {
                        values[valueIdx] = values[valueIdx] * 10 + (b - '0');
                    }
                }
                lastMouse_.button = values[0] & 0x3;
                lastMouse_.isWheel = (values[0] & 0x40) != 0;
                lastMouse_.isDrag = (values[0] & 0x20) != 0;
                lastMouse_.wheelDelta = lastMouse_.isWheel ? ((values[0] & 0x1) ? 1 : -1) : 0;
                lastMouse_.x = values[1] - 1;
                lastMouse_.y = values[2] - 1;
                lastMouse_.pressed = (finalCh == 'M');
                return MOUSE_EVENT;
            }
            if (seq[1] >= '0' && seq[1] <= '9') {
                std::string numPart;
                numPart.push_back(seq[1]);
                int nextByte = readByteBlocking();
                while (nextByte >= '0' && nextByte <= '9') {
                    numPart.push_back(static_cast<char>(nextByte));
                    nextByte = readByteBlocking();
                }
                if (nextByte == '~') {
                    int code = std::atoi(numPart.c_str());
                    switch (code) {
                        case 1: return HOME_KEY;
                        case 3: return DEL_KEY;
                        case 4: return END_KEY;
                        case 5: return PAGE_UP;
                        case 6: return PAGE_DOWN;
                        case 7: return HOME_KEY;
                        case 8: return END_KEY;
                    }
                    return '\x1b';
                }
                if (nextByte == ';') {
                    std::string modPart;
                    int b2 = readByteBlocking();
                    while (b2 >= '0' && b2 <= '9') {
                        modPart.push_back(static_cast<char>(b2));
                        b2 = readByteBlocking();
                    }
                    int modifier = modPart.empty() ? 1 : std::atoi(modPart.c_str());
                    bool ctrlHeld = (modifier == 5) || (modifier == 6);
                    switch (b2) {
                        case 'A': return ctrlHeld ? CTRL_ARROW_UP : ARROW_UP;
                        case 'B': return ctrlHeld ? CTRL_ARROW_DOWN : ARROW_DOWN;
                        case 'C': return ctrlHeld ? CTRL_ARROW_RIGHT : ARROW_RIGHT;
                        case 'D': return ctrlHeld ? CTRL_ARROW_LEFT : ARROW_LEFT;
                        case 'H': return HOME_KEY;
                        case 'F': return END_KEY;
                    }
                    return '\x1b';
                }
                return '\x1b';
            } else {
                switch (seq[1]) {
                    case 'A': return ARROW_UP;
                    case 'B': return ARROW_DOWN;
                    case 'C': return ARROW_RIGHT;
                    case 'D': return ARROW_LEFT;
                    case 'H': return HOME_KEY;
                    case 'F': return END_KEY;
                }
            }
        } else if (seq[0] == 'O') {
            switch (seq[1]) {
                case 'H': return HOME_KEY;
                case 'F': return END_KEY;
            }
        }
        return '\x1b';
    }

    int len = sequenceLengthForLead(uc);
    if (len == 1) {
        return uc;
    }
    uint32_t cp = decodeUtf8Sequence(uc, len);
    return encodeCodepointAsKey(cp);
}

}
