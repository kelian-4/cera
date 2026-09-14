#include "model/Row.hpp"

namespace cera::model {

Row::Row(std::string chars) : chars_(std::move(chars)) {
    updateRender();
}

void Row::setChars(std::string chars) {
    chars_ = std::move(chars);
    updateRender();
}

void Row::insertChar(int at, char c) {
    if (at < 0 || at > size()) at = size();
    chars_.insert(chars_.begin() + at, c);
    updateRender();
}

void Row::delChar(int at) {
    if (at < 0 || at >= size()) return;
    chars_.erase(chars_.begin() + at);
    updateRender();
}

void Row::appendString(const std::string& s) {
    chars_.append(s);
    updateRender();
}

int Row::cxToRx(int cx) const {
    int rx = 0;
    for (int j = 0; j < cx && j < size(); j++) {
        if (chars_[j] == '\t') {
            rx += (kTabStop - 1) - (rx % kTabStop);
        }
        rx++;
    }
    return rx;
}

int Row::rxToCx(int rx) const {
    int cur_rx = 0;
    int cx;
    for (cx = 0; cx < size(); cx++) {
        if (chars_[cx] == '\t') {
            cur_rx += (kTabStop - 1) - (cur_rx % kTabStop);
        }
        cur_rx++;
        if (cur_rx > rx) return cx;
    }
    return cx;
}

void Row::updateRender() {
    render_.clear();
    render_.reserve(chars_.size());

    for (char c : chars_) {
        if (c == '\t') {
            render_.push_back(' ');
            while (render_.size() % kTabStop != 0) {
                render_.push_back(' ');
            }
        } else {
            render_.push_back(c);
        }
    }

    hl_.assign(render_.size(), Highlight::Normal);
}

}
