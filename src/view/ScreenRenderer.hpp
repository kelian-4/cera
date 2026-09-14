#pragma once

#include <string>
#include <vector>

#include "editor/BufferManager.hpp"
#include "io/Config.hpp"
#include "model/Cursor.hpp"
#include "model/Selection.hpp"
#include "syntax/Syntax.hpp"
#include "terminal/Terminal.hpp"
#include "view/MessageKind.hpp"

namespace cera::view {

enum class OverlayMode { None, Help, FullDoc };

struct SearchDisplayInfo {
    bool active = false;
    std::string term;
    int currentIndex = 0;
    int totalMatches = 0;
};

struct ViewState {
    const editor::BufferManager& bufferManager;
    int rowoff;
    int coloff;
    int screenrows;
    int screencols;
    std::string statusMessage;
    MessageKind statusMessageKind;
    long statusMessageAgeSeconds;
    OverlayMode overlay;
    bool showHelpBar;
    SearchDisplayInfo search;
    const io::Config& config;
};

class AppendBuffer {
public:
    void append(const char* s, size_t len) { data_.append(s, len); }
    void append(const std::string& s) { data_.append(s); }
    const std::string& data() const { return data_; }

private:
    std::string data_;
};

class ScreenRenderer {
public:
    explicit ScreenRenderer(terminal::Terminal& term) : term_(term) {}

    void refresh(const ViewState& state);

    int gutterWidth(const ViewState& state) const;
    int textAreaScreenRows(const ViewState& state) const;

private:
    void drawTabBar(AppendBuffer& ab, const ViewState& state);
    void drawRows(AppendBuffer& ab, const ViewState& state);
    void drawStatusBar(AppendBuffer& ab, const ViewState& state);
    void drawHelpBar(AppendBuffer& ab, const ViewState& state);
    void drawMessageBar(AppendBuffer& ab, const ViewState& state);
    void drawFullDoc(AppendBuffer& ab, const ViewState& state);

    terminal::Terminal& term_;
};

}
