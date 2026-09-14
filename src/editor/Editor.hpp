#pragma once

#include <ctime>
#include <functional>
#include <string>
#include <vector>

#include "model/Buffer.hpp"
#include "model/Cursor.hpp"
#include "syntax/Syntax.hpp"
#include "terminal/Terminal.hpp"
#include "view/ScreenRenderer.hpp"

namespace cera::editor {

using PromptCallback = std::function<void(const std::string&, int)>;

class Editor {
public:
    Editor();

    void openFile(const std::string& filename);

    [[noreturn]] void run();

    static constexpr int kQuitTimes = 3;

private:

    void refreshScreen();
    void processKeypress();
    void setStatusMessage(const std::string& msg);

    void moveCursor(int key);

    void insertChar(int c);
    void insertNewline();
    void deleteChar();

    void save();

    void find();
    void findCallback(const std::string& query, int key);

    std::string prompt(const std::string& promptFmt, PromptCallback callback);

    void scroll();

    terminal::Terminal terminal_;
    view::ScreenRenderer renderer_;

    model::Buffer buffer_;
    model::Cursor cursor_;
    int rowoff_ = 0;
    int coloff_ = 0;
    int screenrows_ = 0;
    int screencols_ = 0;

    const syntax::Syntax* activeSyntax_ = nullptr;

    std::string statusMessage_;
    time_t statusMessageTime_ = 0;

    int quitTimes_ = kQuitTimes;

    int findLastMatch_ = -1;
    int findDirection_ = 1;
    int findSavedHlLine_ = -1;
    std::vector<model::Highlight> findSavedHl_;
};

}
