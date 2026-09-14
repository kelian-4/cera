#pragma once

#include <string>

#include "model/Buffer.hpp"
#include "model/Cursor.hpp"
#include "syntax/Syntax.hpp"
#include "terminal/Terminal.hpp"

namespace cera::view {

struct ViewState {
    const model::Buffer& buffer;
    const model::Cursor& cursor;
    int rowoff;
    int coloff;
    int screenrows;
    int screencols;
    const syntax::Syntax* activeSyntax;
    std::string statusMessage;
    long statusMessageAgeSeconds;
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

private:
    void drawRows(AppendBuffer& ab, const ViewState& state);
    void drawStatusBar(AppendBuffer& ab, const ViewState& state);
    void drawMessageBar(AppendBuffer& ab, const ViewState& state);

    terminal::Terminal& term_;
};

}
