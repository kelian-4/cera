#include "editor/Editor.hpp"

#include <cctype>
#include <cstdarg>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

#include "io/FileIO.hpp"

namespace cera::editor {

using terminal::EditorKey;

Editor::Editor() : renderer_(terminal_) {
    if (!terminal_.getWindowSize(&screenrows_, &screencols_)) {
        terminal::die("getWindowSize");
    }
    screenrows_ -= 2;
}

void Editor::openFile(const std::string& filename) {
    io::openFile(buffer_, filename);
    activeSyntax_ = syntax::selectForFilename(filename);
    if (activeSyntax_) {
        for (int i = 0; i < buffer_.numRows(); i++) {
            syntax::updateSyntax(buffer_, buffer_.row(i), activeSyntax_);
        }
    }
}

void Editor::setStatusMessage(const std::string& msg) {
    statusMessage_ = msg;
    statusMessageTime_ = time(nullptr);
}

void Editor::scroll() {
    cursor_.rx = 0;
    if (cursor_.cy < buffer_.numRows()) {
        cursor_.rx = buffer_.row(cursor_.cy).cxToRx(cursor_.cx);
    }
    if (cursor_.cy < rowoff_) {
        rowoff_ = cursor_.cy;
    }
    if (cursor_.cy >= rowoff_ + screenrows_) {
        rowoff_ = cursor_.cy - screenrows_ + 1;
    }
    if (cursor_.rx < coloff_) {
        coloff_ = cursor_.rx;
    }
    if (cursor_.rx >= coloff_ + screencols_) {
        coloff_ = cursor_.rx - screencols_ + 1;
    }
}

void Editor::refreshScreen() {
    scroll();

    view::ViewState state{
        buffer_,
        cursor_,
        rowoff_,
        coloff_,
        screenrows_,
        screencols_,
        activeSyntax_,
        statusMessage_,
        time(nullptr) - statusMessageTime_,
    };
    renderer_.refresh(state);
}

void Editor::moveCursor(int key) {
    bool hasRow = cursor_.cy < buffer_.numRows();

    switch (key) {
        case EditorKey::ARROW_LEFT:
            if (cursor_.cx != 0) {
                cursor_.cx--;
            } else if (cursor_.cy > 0) {
                cursor_.cy--;
                cursor_.cx = buffer_.row(cursor_.cy).size();
            }
            break;
        case EditorKey::ARROW_RIGHT:
            if (hasRow && cursor_.cx < buffer_.row(cursor_.cy).size()) {
                cursor_.cx++;
            } else if (hasRow && cursor_.cx == buffer_.row(cursor_.cy).size()) {
                cursor_.cy++;
                cursor_.cx = 0;
            }
            break;
        case EditorKey::ARROW_UP:
            if (cursor_.cy != 0) cursor_.cy--;
            break;
        case EditorKey::ARROW_DOWN:
            if (cursor_.cy < buffer_.numRows()) cursor_.cy++;
            break;
        default:
            break;
    }

    hasRow = cursor_.cy < buffer_.numRows();
    int rowlen = hasRow ? buffer_.row(cursor_.cy).size() : 0;
    if (cursor_.cx > rowlen) cursor_.cx = rowlen;
}

void Editor::insertChar(int c) {
    buffer_.insertChar(cursor_.cy, cursor_.cx, c);
    if (activeSyntax_) {
        syntax::updateSyntax(buffer_, buffer_.row(cursor_.cy), activeSyntax_);
    }
    cursor_.cx++;
}

void Editor::insertNewline() {
    buffer_.insertNewline(cursor_.cy, cursor_.cx);
    if (activeSyntax_) {
        syntax::updateSyntax(buffer_, buffer_.row(cursor_.cy), activeSyntax_);
        if (cursor_.cy + 1 < buffer_.numRows()) {
            syntax::updateSyntax(buffer_, buffer_.row(cursor_.cy + 1), activeSyntax_);
        }
    }
    cursor_.cy++;
    cursor_.cx = 0;
}

void Editor::deleteChar() {
    if (cursor_.cy == buffer_.numRows()) return;
    if (cursor_.cx == 0 && cursor_.cy == 0) return;

    auto pos = buffer_.deleteChar(cursor_.cy, cursor_.cx);
    cursor_.cy = pos.row;
    cursor_.cx = pos.col;

    if (activeSyntax_ && cursor_.cy < buffer_.numRows()) {
        syntax::updateSyntax(buffer_, buffer_.row(cursor_.cy), activeSyntax_);
    }
}

void Editor::save() {
    if (!buffer_.filename()) {
        std::string filename = prompt("Save as: %s (ESC to cancel)", nullptr);
        if (filename.empty()) {
            setStatusMessage("Save aborted");
            return;
        }
        buffer_.setFilename(filename);
        activeSyntax_ = syntax::selectForFilename(filename);
        if (activeSyntax_) {
            for (int i = 0; i < buffer_.numRows(); i++) {
                syntax::updateSyntax(buffer_, buffer_.row(i), activeSyntax_);
            }
        }
    }

    int err = 0;
    long len = io::saveFile(buffer_, &err);
    if (len >= 0) {
        char msg[80];
        snprintf(msg, sizeof(msg), "%ld bytes written to disk", len);
        setStatusMessage(msg);
    } else {
        char msg[128];
        snprintf(msg, sizeof(msg), "Can't save! I/O error: %s", strerror(err));
        setStatusMessage(msg);
    }
}

void Editor::findCallback(const std::string& query, int key) {
    if (findSavedHlLine_ != -1) {
        buffer_.row(findSavedHlLine_).hl() = findSavedHl_;
        findSavedHl_.clear();
        findSavedHlLine_ = -1;
    }

    if (key == '\r' || key == '\x1b') {
        findLastMatch_ = -1;
        findDirection_ = 1;
        return;
    }
    if (key == EditorKey::ARROW_RIGHT || key == EditorKey::ARROW_DOWN) {
        findDirection_ = 1;
    } else if (key == EditorKey::ARROW_LEFT || key == EditorKey::ARROW_UP) {
        findDirection_ = -1;
    } else {
        findLastMatch_ = -1;
        findDirection_ = 1;
    }

    if (findLastMatch_ == -1) findDirection_ = 1;
    int current = findLastMatch_;

    if (query.empty()) return;

    for (int i = 0; i < buffer_.numRows(); i++) {
        current += findDirection_;
        if (current == -1) current = buffer_.numRows() - 1;
        else if (current == buffer_.numRows()) current = 0;

        model::Row& row = buffer_.row(current);
        size_t matchPos = row.render().find(query);
        if (matchPos != std::string::npos) {
            findLastMatch_ = current;
            cursor_.cy = current;
            cursor_.cx = row.rxToCx(static_cast<int>(matchPos));
            rowoff_ = buffer_.numRows();

            findSavedHlLine_ = current;
            findSavedHl_ = row.hl();
            for (size_t k = 0; k < query.size(); k++) {
                row.hl()[matchPos + k] = model::Highlight::Match;
            }
            break;
        }
    }
}

void Editor::find() {
    int savedCx = cursor_.cx;
    int savedCy = cursor_.cy;
    int savedColoff = coloff_;
    int savedRowoff = rowoff_;

    std::string query = prompt(
        "Search: %s (Use ESC/Arrows/Enter)",
        [this](const std::string& q, int key) { findCallback(q, key); });

    if (query.empty()) {
        cursor_.cx = savedCx;
        cursor_.cy = savedCy;
        coloff_ = savedColoff;
        rowoff_ = savedRowoff;
    }
}

std::string Editor::prompt(const std::string& promptFmt, PromptCallback callback) {
    std::string buf;

    while (true) {
        char msg[256];
        snprintf(msg, sizeof(msg), promptFmt.c_str(), buf.c_str());
        setStatusMessage(msg);
        refreshScreen();

        int c = terminal_.readKey();

        if (c == EditorKey::DEL_KEY || c == ('h' & 0x1f) || c == EditorKey::BACKSPACE) {
            if (!buf.empty()) buf.pop_back();
        } else if (c == '\x1b') {
            setStatusMessage("");
            if (callback) callback(buf, c);
            return "";
        } else if (c == '\r') {
            if (!buf.empty()) {
                setStatusMessage("");
                if (callback) callback(buf, c);
                return buf;
            }
        } else if (!std::iscntrl(c) && c < 128) {
            buf.push_back(static_cast<char>(c));
        }

        if (callback) callback(buf, c);
    }
}

void Editor::processKeypress() {
    int c = terminal_.readKey();

    switch (c) {
        case '\r':
            insertNewline();
            break;

        case 'q' & 0x1f:
            if (buffer_.dirty() && quitTimes_ > 0) {
                char msg[128];
                snprintf(msg, sizeof(msg),
                         "WARNING!!! File has unsaved changes. "
                         "Press Ctrl-Q %d more times to quit.",
                         quitTimes_);
                setStatusMessage(msg);
                quitTimes_--;
                return;
            }
            terminal_.write("\x1b[?1049l", 8);
            std::exit(0);

        case 's' & 0x1f:
            save();
            break;

        case EditorKey::HOME_KEY:
            cursor_.cx = 0;
            break;

        case EditorKey::END_KEY:
            if (cursor_.cy < buffer_.numRows()) {
                cursor_.cx = buffer_.row(cursor_.cy).size();
            }
            break;

        case 'f' & 0x1f:
            find();
            break;

        case EditorKey::BACKSPACE:
        case 'h' & 0x1f:
        case EditorKey::DEL_KEY:
            if (c == EditorKey::DEL_KEY) moveCursor(EditorKey::ARROW_RIGHT);
            deleteChar();
            break;

        case EditorKey::PAGE_UP:
        case EditorKey::PAGE_DOWN: {
            if (c == EditorKey::PAGE_UP) {
                cursor_.cy = rowoff_;
            } else {
                cursor_.cy = rowoff_ + screenrows_ - 1;
                if (cursor_.cy > buffer_.numRows()) cursor_.cy = buffer_.numRows();
            }
            int times = screenrows_;
            while (times--) {
                moveCursor(c == EditorKey::PAGE_UP ? EditorKey::ARROW_UP
                                                    : EditorKey::ARROW_DOWN);
            }
            break;
        }

        case EditorKey::ARROW_UP:
        case EditorKey::ARROW_DOWN:
        case EditorKey::ARROW_LEFT:
        case EditorKey::ARROW_RIGHT:
            moveCursor(c);
            break;

        case 'l' & 0x1f:
        case '\x1b':
            break;

        default:
            insertChar(c);
            break;
    }

    quitTimes_ = kQuitTimes;
}

void Editor::run() {
    setStatusMessage("HELP: Ctrl-S = save | Ctrl-Q = quit | Ctrl-F = find");

    while (true) {
        refreshScreen();
        processKeypress();
    }
}

}
