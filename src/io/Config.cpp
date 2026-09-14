#include "io/Config.hpp"

#include <cstdlib>
#include <fstream>
#include <sstream>

#include "io/Json.hpp"

namespace cera::io {

namespace {

uint32_t parseHexColor(const std::string& s, uint32_t fallback) {
    if (s.empty()) return fallback;
    std::string hex = s;
    if (hex[0] == '#') hex = hex.substr(1);
    if (hex.size() != 6) return fallback;
    char* end = nullptr;
    unsigned long v = std::strtoul(hex.c_str(), &end, 16);
    if (end == hex.c_str()) return fallback;
    return static_cast<uint32_t>(v);
}

void applyThemeField(const json::Value* themeObj, const char* key, uint32_t& target) {
    if (!themeObj) return;
    const json::Value* v = themeObj->get(key);
    if (v && v->isString()) {
        target = parseHexColor(v->asString(), target);
    }
}

}

std::string Config::defaultConfigPath() {
    const char* xdg = std::getenv("XDG_CONFIG_HOME");
    if (xdg && *xdg) {
        return std::string(xdg) + "/cera/config.json";
    }
    const char* home = std::getenv("HOME");
    if (home && *home) {
        return std::string(home) + "/.config/cera/config.json";
    }
    return ".cera.json";
}

Config Config::loadFromFile(const std::string& path, bool* loadedFromFile) {
    Config cfg;
    if (loadedFromFile) *loadedFromFile = false;

    std::ifstream file(path);
    if (!file) return cfg;

    std::ostringstream ss;
    ss << file.rdbuf();
    std::string content = ss.str();

    auto result = json::parse(content);
    if (!result.ok || !result.value->isObject()) return cfg;

    const json::Value& root = *result.value;

    if (const json::Value* v = root.get("tab_stop"); v && v->isNumber()) {
        cfg.tabStop = v->asInt(cfg.tabStop);
    }
    if (const json::Value* v = root.get("quit_times"); v && v->isNumber()) {
        cfg.quitTimes = v->asInt(cfg.quitTimes);
    }
    if (const json::Value* v = root.get("true_color"); v && v->isBool()) {
        cfg.trueColor = v->asBool(cfg.trueColor);
    }
    if (const json::Value* v = root.get("show_line_numbers"); v && v->isBool()) {
        cfg.showLineNumbers = v->asBool(cfg.showLineNumbers);
    }
    if (const json::Value* v = root.get("mouse_enabled"); v && v->isBool()) {
        cfg.mouseEnabled = v->asBool(cfg.mouseEnabled);
    }

    const json::Value* theme = root.get("theme");
    if (theme && theme->isObject()) {
        applyThemeField(theme, "normal", cfg.theme.normal);
        applyThemeField(theme, "comment", cfg.theme.comment);
        applyThemeField(theme, "keyword1", cfg.theme.keyword1);
        applyThemeField(theme, "keyword2", cfg.theme.keyword2);
        applyThemeField(theme, "string", cfg.theme.stringColor);
        applyThemeField(theme, "number", cfg.theme.number);
        applyThemeField(theme, "match", cfg.theme.match);
        applyThemeField(theme, "status_bar_fg", cfg.theme.statusBarFg);
        applyThemeField(theme, "status_bar_bg", cfg.theme.statusBarBg);
        applyThemeField(theme, "line_number_fg", cfg.theme.lineNumberFg);
        applyThemeField(theme, "error_fg", cfg.theme.errorFg);
        applyThemeField(theme, "success_fg", cfg.theme.successFg);
    }

    if (loadedFromFile) *loadedFromFile = true;
    return cfg;
}

}
