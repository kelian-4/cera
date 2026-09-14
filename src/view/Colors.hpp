#pragma once

#include <cstdint>
#include <string>

#include "io/Config.hpp"
#include "model/Row.hpp"

namespace cera::view {

uint32_t highlightToRgb(model::Highlight hl, const io::ThemeColors& theme);

std::string fgSequence(uint32_t rgb, bool trueColor);
std::string bgSequence(uint32_t rgb, bool trueColor);
std::string resetFgSequence();
std::string resetBgSequence();

int rgbToAnsi256(uint32_t rgb);

}
