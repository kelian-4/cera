#pragma once

#include <cstddef>
#include <termios.h>

namespace cera::terminal {

enum EditorKey {
    BACKSPACE = 127,
    ARROW_LEFT = 1000,
    ARROW_RIGHT,
    ARROW_UP,
    ARROW_DOWN,
    DEL_KEY,
    HOME_KEY,
    END_KEY,
    PAGE_UP,
    PAGE_DOWN
};

class Terminal {
public:
    Terminal();
    ~Terminal();

    Terminal(const Terminal&) = delete;
    Terminal& operator=(const Terminal&) = delete;

    bool getWindowSize(int* rows, int* cols);

    int readKey();

    void write(const char* buf, size_t len);

private:
    void enableRawMode();
    void disableRawMode();
    bool getCursorPosition(int* rows, int* cols);

    struct termios orig_termios_{};
    bool raw_mode_enabled_ = false;
};

[[noreturn]] void die(const char* s);

}
