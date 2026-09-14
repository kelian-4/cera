#pragma once

#include <string>

namespace cera::editor {

class Clipboard {
public:
    Clipboard();

    bool copy(const std::string& text);

    std::string paste();

    bool usingSystemClipboard() const { return systemToolAvailable_; }
    const std::string& toolName() const { return toolName_; }

private:
    bool systemToolAvailable_ = false;
    std::string toolName_;
    std::string copyCommand_;
    std::string pasteCommand_;
    std::string fallback_;

    void detectTool();
};

}
