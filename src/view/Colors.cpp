#include "view/Colors.hpp"

#include <algorithm>
#include <cstdio>

namespace cera::view {

uint32_t highlightToRgb(model::Highlight hl, const io::ThemeColors& theme) {
    switch (hl) {
        case model::Highlight::Comment:
        case model::Highlight::MLComment:
            return theme.comment;
        case model::Highlight::Keyword1:
            return theme.keyword1;
        case model::Highlight::Keyword2:
            return theme.keyword2;
        case model::Highlight::String:
            return theme.stringColor;
        case model::Highlight::Number:
            return theme.number;
        case model::Highlight::Match:
            return theme.match;
        case model::Highlight::Normal:
        default:
            return theme.normal;
    }
}

namespace {

int clamp6(int v) {
    return std::clamp(v, 0, 5);
}

}

int rgbToAnsi256(uint32_t rgb) {
    int r = static_cast<int>((rgb >> 16) & 0xFF);
    int g = static_cast<int>((rgb >> 8) & 0xFF);
    int b = static_cast<int>(rgb & 0xFF);

    int rIdx = clamp6((r * 5 + 127) / 255);
    int gIdx = clamp6((g * 5 + 127) / 255);
    int bIdx = clamp6((b * 5 + 127) / 255);

    return 16 + 36 * rIdx + 6 * gIdx + bIdx;
}

std::string fgSequence(uint32_t rgb, bool trueColor) {
    char buf[32];
    if (trueColor) {
        int r = static_cast<int>((rgb >> 16) & 0xFF);
        int g = static_cast<int>((rgb >> 8) & 0xFF);
        int b = static_cast<int>(rgb & 0xFF);
        int len = snprintf(buf, sizeof(buf), "\x1b[38;2;%d;%d;%dm", r, g, b);
        return std::string(buf, static_cast<size_t>(len));
    }
    int idx = rgbToAnsi256(rgb);
    int len = snprintf(buf, sizeof(buf), "\x1b[38;5;%dm", idx);
    return std::string(buf, static_cast<size_t>(len));
}

std::string bgSequence(uint32_t rgb, bool trueColor) {
    char buf[32];
    if (trueColor) {
        int r = static_cast<int>((rgb >> 16) & 0xFF);
        int g = static_cast<int>((rgb >> 8) & 0xFF);
        int b = static_cast<int>(rgb & 0xFF);
        int len = snprintf(buf, sizeof(buf), "\x1b[48;2;%d;%d;%dm", r, g, b);
        return std::string(buf, static_cast<size_t>(len));
    }
    int idx = rgbToAnsi256(rgb);
    int len = snprintf(buf, sizeof(buf), "\x1b[48;5;%dm", idx);
    return std::string(buf, static_cast<size_t>(len));
}

std::string resetFgSequence() {
    return "\x1b[39m";
}

std::string resetBgSequence() {
    return "\x1b[49m";
}

}
