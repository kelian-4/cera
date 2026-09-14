#pragma once

#include <cstddef>
#include <cstdint>
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
    PAGE_DOWN,
    MOUSE_EVENT,
    CTRL_ARROW_LEFT,
    CTRL_ARROW_RIGHT,
    CTRL_ARROW_UP,
    CTRL_ARROW_DOWN
};

constexpr uint32_t kCodepointBase = 0x110000;

inline bool isEncodedCodepoint(int key) {
    return key >= static_cast<int>(kCodepointBase);
}

inline uint32_t decodeCodepoint(int key) {
    return static_cast<uint32_t>(key) - kCodepointBase;
}

inline int encodeCodepointAsKey(uint32_t codepoint) {
    return static_cast<int>(kCodepointBase + codepoint);
}

struct MouseEvent {
    int button;
    int x;
    int y;
    bool pressed;
    bool isWheel;
    int wheelDelta;
};

class Terminal {
public:
    Terminal();
    ~Terminal();

    Terminal(const Terminal&) = delete;
    Terminal& operator=(const Terminal&) = delete;

    bool getWindowSize(int* rows, int* cols);

    static void installResizeHandler();
    static bool consumeResizeFlag();

    int readKey();

    const MouseEvent& lastMouseEvent() const { return lastMouse_; }

    void write(const char* buf, size_t len);

    void enableMouseReporting();
    void disableMouseReporting();

private:
    void enableRawMode();
    void disableRawMode();
    bool getCursorPosition(int* rows, int* cols);

    struct termios orig_termios_{};
    bool raw_mode_enabled_ = false;
    MouseEvent lastMouse_{};
};

[[noreturn]] void die(const char* s);

}
