#pragma once

#include <ctime>
#include <functional>
#include <string>
#include <vector>

#include "editor/BufferManager.hpp"
#include "editor/Clipboard.hpp"
#include "io/Config.hpp"
#include "model/Selection.hpp"
#include "terminal/Terminal.hpp"
#include "view/MessageKind.hpp"
#include "view/ScreenRenderer.hpp"

namespace cera::editor {

using PromptCallback = std::function<void(const std::string&, int)>;

enum class ReplaceState { Inactive, AwaitingConfirm };

class Editor {
public:
    Editor();

    void openFile(const std::string& filename);

    [[noreturn]] void run();

private:
    void refreshScreen();
    void processKeypress();
    void setStatusMessage(const std::string& msg, view::MessageKind kind = view::MessageKind::Info);

    void moveCursor(int key, bool extendSelection);
    void handleMouseEvent(const terminal::MouseEvent& ev);

    void insertCodepoint(uint32_t codepoint);
    void insertNewline();
    void deleteCharBackward();
    void deleteCharForward();
    void deleteSelection();
    void deleteSelectionForUndo(model::Position from, model::Position to);
    std::string textInRange(model::Position from, model::Position to) const;
    void insertTextAt(model::Position pos, const std::string& text, model::Position* endOut);

    void save();

    void find();
    void findNext(int direction);
    void updateFindHighlight();
    void clearFindHighlight();

    void replace();

    void gotoLine();

    void undo();
    void redo();

    void copySelection();
    void cutSelection();
    void pasteClipboard();

    void toggleHelpBar();
    void toggleFullDoc();

    std::string prompt(const std::string& promptFmt, PromptCallback callback);

    void scroll();
    void applyResizeIfNeeded();

    void recordAction(EditAction action);

    terminal::Terminal terminal_;
    view::ScreenRenderer renderer_;

    BufferManager bufferManager_;
    Clipboard clipboard_;
    io::Config config_;

    int screenrows_ = 0;
    int screencols_ = 0;

    std::string statusMessage_;
    view::MessageKind statusMessageKind_ = view::MessageKind::Info;
    time_t statusMessageTime_ = 0;

    int quitTimes_ = 0;

    bool showHelpBar_ = false;
    view::OverlayMode overlay_ = view::OverlayMode::None;

    bool searchActive_ = false;
    std::string searchTerm_;
    int searchCurrentMatch_ = 0;
    int searchTotalMatches_ = 0;
    model::Position searchSavedCursor_{};
    int searchSavedRowoff_ = 0;
    int searchSavedColoff_ = 0;

    ReplaceState replaceState_ = ReplaceState::Inactive;
    std::string replaceSearchTerm_;
    std::string replaceWithTerm_;
    model::Position replaceCursor_{};
};

}
