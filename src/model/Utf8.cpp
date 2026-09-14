#include "model/Utf8.hpp"

#include <cwchar>

namespace cera::model::utf8 {

int sequenceLength(unsigned char leadByte) {
    if ((leadByte & 0x80) == 0x00) return 1;
    if ((leadByte & 0xE0) == 0xC0) return 2;
    if ((leadByte & 0xF0) == 0xE0) return 3;
    if ((leadByte & 0xF8) == 0xF0) return 4;
    return 1;
}

DecodedChar decode(const std::string& s, size_t byteOffset) {
    if (byteOffset >= s.size()) return {0, 0};

    unsigned char lead = static_cast<unsigned char>(s[byteOffset]);
    int len = sequenceLength(lead);
    if (byteOffset + static_cast<size_t>(len) > s.size()) len = 1;

    uint32_t cp = 0;
    if (len == 1) {
        cp = lead;
    } else if (len == 2) {
        cp = static_cast<uint32_t>(lead & 0x1F);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 1]) & 0x3F);
    } else if (len == 3) {
        cp = static_cast<uint32_t>(lead & 0x0F);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 1]) & 0x3F);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 2]) & 0x3F);
    } else if (len == 4) {
        cp = static_cast<uint32_t>(lead & 0x07);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 1]) & 0x3F);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 2]) & 0x3F);
        cp = (cp << 6) | (static_cast<unsigned char>(s[byteOffset + 3]) & 0x3F);
    }

    return {cp, len};
}

int codepointCount(const std::string& s) {
    int count = 0;
    size_t offset = 0;
    while (offset < s.size()) {
        DecodedChar dc = decode(s, offset);
        offset += static_cast<size_t>(dc.byteLength);
        count++;
    }
    return count;
}

std::vector<size_t> byteOffsets(const std::string& s) {
    std::vector<size_t> offsets;
    size_t offset = 0;
    while (offset < s.size()) {
        offsets.push_back(offset);
        DecodedChar dc = decode(s, offset);
        offset += static_cast<size_t>(dc.byteLength);
    }
    offsets.push_back(s.size());
    return offsets;
}

size_t byteOffsetForCharIndex(const std::string& s, int charIndex) {
    if (charIndex <= 0) return 0;
    size_t offset = 0;
    int count = 0;
    while (offset < s.size() && count < charIndex) {
        DecodedChar dc = decode(s, offset);
        offset += static_cast<size_t>(dc.byteLength);
        count++;
    }
    return offset;
}

int charIndexForByteOffset(const std::string& s, size_t byteOffset) {
    int count = 0;
    size_t offset = 0;
    while (offset < byteOffset && offset < s.size()) {
        DecodedChar dc = decode(s, offset);
        offset += static_cast<size_t>(dc.byteLength);
        count++;
    }
    return count;
}

int displayWidth(uint32_t codepoint) {
    if (codepoint == 0) return 0;
    wchar_t wc = static_cast<wchar_t>(codepoint);
    int w = wcwidth(wc);
    if (w < 0) return 1;
    return w;
}

std::string encode(uint32_t codepoint) {
    std::string out;
    if (codepoint <= 0x7F) {
        out.push_back(static_cast<char>(codepoint));
    } else if (codepoint <= 0x7FF) {
        out.push_back(static_cast<char>(0xC0 | (codepoint >> 6)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    } else if (codepoint <= 0xFFFF) {
        out.push_back(static_cast<char>(0xE0 | (codepoint >> 12)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 6) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    } else {
        out.push_back(static_cast<char>(0xF0 | (codepoint >> 18)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 12) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | ((codepoint >> 6) & 0x3F)));
        out.push_back(static_cast<char>(0x80 | (codepoint & 0x3F)));
    }
    return out;
}

}
