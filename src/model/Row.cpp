#include "model/Row.hpp"

#include "model/Utf8.hpp"

namespace cera::model {

int Row::tabStop = 8;

Row::Row(std::string chars) : chars_(std::move(chars)) {
    charCount_ = utf8::codepointCount(chars_);
    updateRender();
}

void Row::setChars(std::string chars) {
    chars_ = std::move(chars);
    charCount_ = utf8::codepointCount(chars_);
    updateRender();
}

size_t Row::byteOffsetOf(int charIdx) const {
    if (charIdx < 0) charIdx = 0;
    if (charIdx > charCount_) charIdx = charCount_;
    return utf8::byteOffsetForCharIndex(chars_, charIdx);
}

void Row::insertChar(int at, uint32_t codepoint) {
    if (at < 0 || at > charCount_) at = charCount_;
    size_t byteOff = byteOffsetOf(at);
    std::string encoded = utf8::encode(codepoint);
    chars_.insert(byteOff, encoded);
    charCount_++;
    updateRender();
}

void Row::delChar(int at) {
    if (at < 0 || at >= charCount_) return;
    size_t from = byteOffsetOf(at);
    size_t to = byteOffsetOf(at + 1);
    chars_.erase(from, to - from);
    charCount_--;
    updateRender();
}

void Row::appendString(const std::string& s) {
    chars_.append(s);
    charCount_ = utf8::codepointCount(chars_);
    updateRender();
}

std::string Row::substrByChar(int fromCharIdx, int toCharIdx) const {
    if (fromCharIdx < 0) fromCharIdx = 0;
    if (toCharIdx > charCount_) toCharIdx = charCount_;
    if (fromCharIdx >= toCharIdx) return "";
    size_t from = byteOffsetOf(fromCharIdx);
    size_t to = byteOffsetOf(toCharIdx);
    return chars_.substr(from, to - from);
}

int Row::cxToRx(int cx) const {
    int rx = 0;
    auto offsets = utf8::byteOffsets(chars_);
    int limit = cx < charCount_ ? cx : charCount_;
    for (int j = 0; j < limit; j++) {
        auto dc = utf8::decode(chars_, offsets[static_cast<size_t>(j)]);
        if (dc.codepoint == '\t') {
            rx += (tabStop - 1) - (rx % tabStop);
            rx++;
        } else {
            rx += utf8::displayWidth(dc.codepoint);
        }
    }
    return rx;
}

int Row::rxToCx(int rx) const {
    int curRx = 0;
    auto offsets = utf8::byteOffsets(chars_);
    int cx;
    for (cx = 0; cx < charCount_; cx++) {
        auto dc = utf8::decode(chars_, offsets[static_cast<size_t>(cx)]);
        if (dc.codepoint == '\t') {
            curRx += (tabStop - 1) - (curRx % tabStop);
            curRx++;
        } else {
            curRx += utf8::displayWidth(dc.codepoint);
        }
        if (curRx > rx) return cx;
    }
    return cx;
}

int Row::renderCharWidth(int renderCharIdx) const {
    if (renderCharIdx < 0 || renderCharIdx >= renderCharCount_) return 0;
    size_t off = renderByteOffsets_[static_cast<size_t>(renderCharIdx)];
    auto dc = utf8::decode(render_, off);
    if (dc.codepoint == ' ') return 1;
    return utf8::displayWidth(dc.codepoint) > 0 ? utf8::displayWidth(dc.codepoint) : 1;
}

void Row::updateRender() {
    render_.clear();
    render_.reserve(chars_.size());

    auto offsets = utf8::byteOffsets(chars_);
    for (int j = 0; j < charCount_; j++) {
        auto dc = utf8::decode(chars_, offsets[static_cast<size_t>(j)]);
        if (dc.codepoint == '\t') {
            size_t col = utf8::codepointCount(render_);
            render_.push_back(' ');
            col++;
            while (col % tabStop != 0) {
                render_.push_back(' ');
                col++;
            }
        } else {
            render_.append(chars_, offsets[static_cast<size_t>(j)], static_cast<size_t>(dc.byteLength));
        }
    }

    renderCharCount_ = utf8::codepointCount(render_);
    renderByteOffsets_ = utf8::byteOffsets(render_);
    hl_.assign(static_cast<size_t>(renderCharCount_), Highlight::Normal);
}

}
