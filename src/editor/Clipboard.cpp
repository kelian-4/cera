#include "editor/Clipboard.hpp"

#include <array>
#include <cstdio>
#include <cstdlib>

namespace cera::editor {

namespace {

bool commandExists(const std::string& name) {
    std::string cmd = "command -v " + name + " > /dev/null 2>&1";
    return std::system(cmd.c_str()) == 0;
}

}

Clipboard::Clipboard() {
    detectTool();
}

void Clipboard::detectTool() {
    if (const char* wayland = std::getenv("WAYLAND_DISPLAY"); wayland && commandExists("wl-copy") && commandExists("wl-paste")) {
        toolName_ = "wl-clipboard";
        copyCommand_ = "wl-copy";
        pasteCommand_ = "wl-paste --no-newline";
        systemToolAvailable_ = true;
        return;
    }
    if (commandExists("xclip")) {
        toolName_ = "xclip";
        copyCommand_ = "xclip -selection clipboard -in 2>/dev/null";
        pasteCommand_ = "xclip -selection clipboard -out 2>/dev/null";
        systemToolAvailable_ = true;
        return;
    }
    if (commandExists("xsel")) {
        toolName_ = "xsel";
        copyCommand_ = "xsel --clipboard --input 2>/dev/null";
        pasteCommand_ = "xsel --clipboard --output 2>/dev/null";
        systemToolAvailable_ = true;
        return;
    }
    if (commandExists("pbcopy") && commandExists("pbpaste")) {
        toolName_ = "pbcopy/pbpaste";
        copyCommand_ = "pbcopy";
        pasteCommand_ = "pbpaste";
        systemToolAvailable_ = true;
        return;
    }

    toolName_ = "interne (aucun outil système trouvé)";
    systemToolAvailable_ = false;
}

bool Clipboard::copy(const std::string& text) {
    fallback_ = text;

    if (!systemToolAvailable_) return false;

    FILE* pipe = popen(copyCommand_.c_str(), "w");
    if (!pipe) {
        systemToolAvailable_ = false;
        return false;
    }

    size_t written = fwrite(text.data(), 1, text.size(), pipe);
    pclose(pipe);

    bool ok = (written == text.size());
    if (!ok) {
        systemToolAvailable_ = false;
    }
    return ok;
}

std::string Clipboard::paste() {
    if (!systemToolAvailable_) return fallback_;

    FILE* pipe = popen(pasteCommand_.c_str(), "r");
    if (!pipe) {
        systemToolAvailable_ = false;
        return fallback_;
    }

    std::string result;
    std::array<char, 4096> buf{};
    size_t n;
    while ((n = fread(buf.data(), 1, buf.size(), pipe)) > 0) {
        result.append(buf.data(), n);
    }
    pclose(pipe);

    if (result.empty() && !fallback_.empty()) {
        return fallback_;
    }

    return result;
}

}
