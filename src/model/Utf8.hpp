#pragma once

#include <cstdint>
#include <string>
#include <vector>

namespace cera::model::utf8 {

struct DecodedChar {
    uint32_t codepoint;
    int byteLength;
};

int sequenceLength(unsigned char leadByte);

DecodedChar decode(const std::string& s, size_t byteOffset);

int codepointCount(const std::string& s);

size_t byteOffsetForCharIndex(const std::string& s, int charIndex);

int charIndexForByteOffset(const std::string& s, size_t byteOffset);

int displayWidth(uint32_t codepoint);

std::string encode(uint32_t codepoint);

std::vector<size_t> byteOffsets(const std::string& s);

}
