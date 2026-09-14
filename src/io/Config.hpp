#pragma once

#include <cstdint>
#include <string>

namespace cera::io {

struct ThemeColors {
    uint32_t normal = 0xD8DEE9;
    uint32_t comment = 0x616E88;
    uint32_t keyword1 = 0x81A1C1;
    uint32_t keyword2 = 0xA3BE8C;
    uint32_t stringColor = 0xEBCB8B;
    uint32_t number = 0xB48EAD;
    uint32_t match = 0xFFFF00;
    uint32_t statusBarFg = 0x2E3440;
    uint32_t statusBarBg = 0x88C0D0;
    uint32_t lineNumberFg = 0x4C566A;
    uint32_t errorFg = 0xBF616A;
    uint32_t successFg = 0xA3BE8C;
};

struct Config {
    int tabStop = 8;
    int quitTimes = 3;
    bool trueColor = true;
    bool showLineNumbers = true;
    bool mouseEnabled = true;
    ThemeColors theme;

    static Config loadFromFile(const std::string& path, bool* loadedFromFile);
    static std::string defaultConfigPath();
};

}
