#include "editor/Editor.hpp"

#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "model/Utf8.hpp"
#include "view/HelpText.hpp"
#include "io/FileIO.hpp"

namespace cera::editor {

using terminal::EditorKey;

namespace {
constexpr int ctrl(char c) { return c & 0x1f; }

model::Position endPositionAfterText(model::Position start, const std::string& text) {
    size_t nlCount = 0;
    for (char c : text) {
        if (c == '\n') nlCount++;
    }
    if (nlCount == 0) {
        return {start.row, start.col + model::utf8::codepointCount(text)};
    }
    std::string lastSegment = text.substr(text.find_last_of('\n') + 1);
    return {start.row + static_cast<int>(nlCount), model::utf8::codepointCount(lastSegment)};
}
}

Editor::Editor() : renderer_(terminal_) {
    setlocale(LC_ALL, "");

    bool loadedCfg = false;
    config_ = io::Config::loadFromFile(io::Config::defaultConfigPath(), &loadedCfg);

    if (!terminal_.getWindowSize(&screenrows_, &screencols_)) {
        terminal::die("getWindowSize");
    }

    quitTimes_ = config_.quitTimes;
    model::Row::tabStop = config_.tabStop > 0 ? config_.tabStop : 8;

    if (config_.mouseEnabled) {
        terminal_.enableMouseReporting();
    }

    terminal::Terminal::installResizeHandler();
}

void Editor::openFile(const std::string& filename) {
    bufferManager_.openFile(filename);
}

void Editor::setStatusMessage(const std::string& msg, view::MessageKind kind) {
    statusMessage_ = msg;
    statusMessageKind_ = kind;
    statusMessageTime_ = time(nullptr);
}

std::string Editor::prompt(const std::string& promptFmt, PromptCallback callback) {
    std::string buf;

    while (true) {
        char msg[256];
        snprintf(msg, sizeof(msg), promptFmt.c_str(), buf.c_str());
        setStatusMessage(msg);
        refreshScreen();

        int c = terminal_.readKey();

        if (c == EditorKey::DEL_KEY || c == ctrl('h') || c == EditorKey::BACKSPACE) {
            if (!buf.empty()) buf.pop_back();
        } else if (c == '\x1b') {
            setStatusMessage("");
            if (callback) callback(buf, c);
            return "";
        } else if (c == '\r') {
            if (!buf.empty()) {
                setStatusMessage("");
                if (callback) callback(buf, c);
                return buf;
            }
        } else if (terminal::isEncodedCodepoint(c)) {
            uint32_t cp = terminal::decodeCodepoint(c);
            buf += model::utf8::encode(cp);
        } else if (!std::iscntrl(c) && c < 128) {
            buf.push_back(static_cast<char>(c));
        }

        if (callback) callback(buf, c);
    }
}

void Editor::save() {
    auto& ob = bufferManager_.active();

    if (!ob.buffer.filename()) {
        std::string filename = prompt("Sauvegarder sous : %s (Echap pour annuler)", nullptr);
        if (filename.empty()) {
            setStatusMessage("Sauvegarde annulee");
            return;
        }
        ob.buffer.setFilename(filename);
        ob.activeSyntax = syntax::selectForFilename(filename);
        if (ob.activeSyntax) {
            for (int i = 0; i < ob.buffer.numRows(); i++) {
                syntax::updateSyntax(ob.buffer, ob.buffer.row(i), ob.activeSyntax);
            }
        }
    }

    int err = 0;
    long len = io::saveFile(ob.buffer, &err);
    if (len >= 0) {
        char msg[80];
        snprintf(msg, sizeof(msg), "%ld octets ecrits sur le disque", len);
        setStatusMessage(msg, view::MessageKind::Success);
    } else {
        char msg[160];
        snprintf(msg, sizeof(msg), "Erreur de sauvegarde : %s", strerror(err));
        setStatusMessage(msg, view::MessageKind::Error);
    }
}

void Editor::applyResizeIfNeeded() {
    if (terminal::Terminal::consumeResizeFlag()) {
        int rows, cols;
        if (terminal_.getWindowSize(&rows, &cols)) {
            screenrows_ = rows;
            screencols_ = cols;
        }
    }
}

void Editor::scroll() {
    auto& ob = bufferManager_.active();
    int textRows = renderer_.textAreaScreenRows(view::ViewState{
        bufferManager_, ob.rowoff, ob.coloff, screenrows_, screencols_,
        statusMessage_, statusMessageKind_, 0, overlay_, showHelpBar_,
        {}, config_});
    int gutter = renderer_.gutterWidth(view::ViewState{
        bufferManager_, ob.rowoff, ob.coloff, screenrows_, screencols_,
        statusMessage_, statusMessageKind_, 0, overlay_, showHelpBar_,
        {}, config_});
    int textCols = screencols_ - gutter;

    ob.cursor.rx = 0;
    if (ob.cursor.cy < ob.buffer.numRows()) {
        ob.cursor.rx = ob.buffer.row(ob.cursor.cy).cxToRx(ob.cursor.cx);
    }
    if (ob.cursor.cy < ob.rowoff) {
        ob.rowoff = ob.cursor.cy;
    }
    if (ob.cursor.cy >= ob.rowoff + textRows) {
        ob.rowoff = ob.cursor.cy - textRows + 1;
    }
    if (ob.cursor.rx < ob.coloff) {
        ob.coloff = ob.cursor.rx;
    }
    if (ob.cursor.rx >= ob.coloff + textCols) {
        ob.coloff = ob.cursor.rx - textCols + 1;
    }
}

void Editor::refreshScreen() {
    applyResizeIfNeeded();
    scroll();

    auto& ob = bufferManager_.active();

    view::SearchDisplayInfo searchInfo;
    searchInfo.active = searchActive_;
    searchInfo.term = searchTerm_;
    searchInfo.currentIndex = searchCurrentMatch_;
    searchInfo.totalMatches = searchTotalMatches_;

    view::ViewState state{
        bufferManager_,
        ob.rowoff,
        ob.coloff,
        screenrows_,
        screencols_,
        statusMessage_,
        statusMessageKind_,
        time(nullptr) - statusMessageTime_,
        overlay_,
        showHelpBar_,
        searchInfo,
        config_,
    };
    renderer_.refresh(state);
}

void Editor::run() {
    setStatusMessage(view::shortHelpLine());

    while (true) {
        refreshScreen();
        processKeypress();
    }
}

void Editor::recordAction(EditAction action) {
    bufferManager_.active().undoStack.push(action);
}

void Editor::moveCursor(int key, bool extendSelection) {
    auto& ob = bufferManager_.active();
    auto& cursor = ob.cursor;

    if (extendSelection && !ob.selection.isActive) {
        ob.selection.isActive = true;
        ob.selection.anchor = {cursor.cy, cursor.cx};
    } else if (!extendSelection) {
        ob.selection.isActive = false;
    }

    bool hasRow = cursor.cy < ob.buffer.numRows();

    switch (key) {
        case EditorKey::ARROW_LEFT:
            if (cursor.cx != 0) {
                cursor.cx--;
            } else if (cursor.cy > 0) {
                cursor.cy--;
                cursor.cx = ob.buffer.row(cursor.cy).size();
            }
            break;
        case EditorKey::ARROW_RIGHT:
            if (hasRow && cursor.cx < ob.buffer.row(cursor.cy).size()) {
                cursor.cx++;
            } else if (hasRow && cursor.cx == ob.buffer.row(cursor.cy).size()) {
                cursor.cy++;
                cursor.cx = 0;
            }
            break;
        case EditorKey::ARROW_UP:
            if (cursor.cy != 0) cursor.cy--;
            break;
        case EditorKey::ARROW_DOWN:
            if (cursor.cy < ob.buffer.numRows()) cursor.cy++;
            break;
        default:
            break;
    }

    hasRow = cursor.cy < ob.buffer.numRows();
    int rowlen = hasRow ? ob.buffer.row(cursor.cy).size() : 0;
    if (cursor.cx > rowlen) cursor.cx = rowlen;

    if (extendSelection) {
        ob.selection.active = {cursor.cy, cursor.cx};
    }

    ob.undoStack.breakCoalescing();
}

std::string Editor::textInRange(model::Position from, model::Position to) const {
    const auto& buf = bufferManager_.active().buffer;
    if (from.row == to.row) {
        return buf.row(from.row).substrByChar(from.col, to.col);
    }
    std::string result = buf.row(from.row).substrByChar(from.col, buf.row(from.row).size());
    result += '\n';
    for (int r = from.row + 1; r < to.row; r++) {
        result += buf.row(r).chars();
        result += '\n';
    }
    result += buf.row(to.row).substrByChar(0, to.col);
    return result;
}

void Editor::insertTextAt(model::Position pos, const std::string& text, model::Position* endOut) {
    auto& buf = bufferManager_.active().buffer;
    int row = pos.row;
    int col = pos.col;

    size_t start = 0;
    while (true) {
        size_t nl = text.find('\n', start);
        std::string segment = (nl == std::string::npos) ? text.substr(start) : text.substr(start, nl - start);

        for (size_t byteOff = 0; byteOff < segment.size();) {
            auto dc = model::utf8::decode(segment, byteOff);
            buf.insertChar(row, col, dc.codepoint);
            col++;
            byteOff += static_cast<size_t>(dc.byteLength);
        }

        if (nl == std::string::npos) break;

        buf.insertNewline(row, col);
        row++;
        col = 0;
        start = nl + 1;
    }

    if (endOut) *endOut = {row, col};

    auto& ob = bufferManager_.active();
    if (ob.activeSyntax) {
        for (int r = pos.row; r <= row && r < buf.numRows(); r++) {
            syntax::updateSyntax(buf, buf.row(r), ob.activeSyntax);
        }
    }
}

void Editor::deleteSelectionForUndo(model::Position from, model::Position to) {
    auto& ob = bufferManager_.active();
    auto& buf = ob.buffer;

    if (from.row == to.row) {
        for (int i = to.col - 1; i >= from.col; i--) {
            buf.deleteChar(from.row, i + 1);
        }
    } else {
        std::string tailAfter = buf.row(to.row).substrByChar(to.col, buf.row(to.row).size());
        std::string head = buf.row(from.row).substrByChar(0, from.col);
        for (int r = to.row; r > from.row; r--) {
            buf.deleteRow(r);
        }
        buf.row(from.row).setChars(head + tailAfter);
    }

    if (ob.activeSyntax && from.row < buf.numRows()) {
        syntax::updateSyntax(buf, buf.row(from.row), ob.activeSyntax);
    }
}

void Editor::deleteSelection() {
    auto& ob = bufferManager_.active();
    if (!ob.selection.isActive || ob.selection.empty()) return;

    model::Position from = ob.selection.start();
    model::Position to = ob.selection.end();
    std::string removed = textInRange(from, to);

    deleteSelectionForUndo(from, to);

    ob.cursor.cy = from.row;
    ob.cursor.cx = from.col;
    ob.selection.isActive = false;

    EditAction action;
    action.start = from;
    action.removedText = removed;
    action.cursorBefore = to;
    action.cursorAfter = from;
    action.kind = EditKind::DeleteBlock;
    recordAction(action);
}

void Editor::insertCodepoint(uint32_t codepoint) {
    auto& ob = bufferManager_.active();

    model::Position before{ob.cursor.cy, ob.cursor.cx};

    if (ob.selection.isActive && !ob.selection.empty()) {
        deleteSelection();
        before = {ob.cursor.cy, ob.cursor.cx};
    }

    if (ob.cursor.cy == ob.buffer.numRows()) {
        ob.buffer.insertRow(ob.buffer.numRows(), "");
    }
    ob.buffer.insertChar(ob.cursor.cy, ob.cursor.cx, codepoint);
    if (ob.activeSyntax) {
        syntax::updateSyntax(ob.buffer, ob.buffer.row(ob.cursor.cy), ob.activeSyntax);
    }
    ob.cursor.cx++;

    EditAction action;
    action.start = before;
    action.insertedText = model::utf8::encode(codepoint);
    action.cursorBefore = before;
    action.cursorAfter = {ob.cursor.cy, ob.cursor.cx};
    action.kind = EditKind::InsertChar;
    recordAction(action);
}

void Editor::insertNewline() {
    auto& ob = bufferManager_.active();

    if (ob.selection.isActive && !ob.selection.empty()) {
        deleteSelection();
    }

    model::Position before{ob.cursor.cy, ob.cursor.cx};

    ob.buffer.insertNewline(ob.cursor.cy, ob.cursor.cx);
    if (ob.activeSyntax) {
        syntax::updateSyntax(ob.buffer, ob.buffer.row(ob.cursor.cy), ob.activeSyntax);
        if (ob.cursor.cy + 1 < ob.buffer.numRows()) {
            syntax::updateSyntax(ob.buffer, ob.buffer.row(ob.cursor.cy + 1), ob.activeSyntax);
        }
    }
    ob.cursor.cy++;
    ob.cursor.cx = 0;

    EditAction action;
    action.start = before;
    action.insertedText = "\n";
    action.cursorBefore = before;
    action.cursorAfter = {ob.cursor.cy, ob.cursor.cx};
    action.kind = EditKind::InsertNewline;
    recordAction(action);
}

void Editor::deleteCharBackward() {
    auto& ob = bufferManager_.active();

    if (ob.selection.isActive && !ob.selection.empty()) {
        deleteSelection();
        return;
    }

    if (ob.cursor.cy == ob.buffer.numRows()) return;
    if (ob.cursor.cx == 0 && ob.cursor.cy == 0) return;

    model::Position before{ob.cursor.cy, ob.cursor.cx};
    std::string removed;
    if (ob.cursor.cx > 0) {
        removed = ob.buffer.row(ob.cursor.cy).substrByChar(ob.cursor.cx - 1, ob.cursor.cx);
    } else {
        removed = "\n";
    }

    auto pos = ob.buffer.deleteChar(ob.cursor.cy, ob.cursor.cx);
    ob.cursor.cy = pos.row;
    ob.cursor.cx = pos.col;

    if (ob.activeSyntax && ob.cursor.cy < ob.buffer.numRows()) {
        syntax::updateSyntax(ob.buffer, ob.buffer.row(ob.cursor.cy), ob.activeSyntax);
    }

    EditAction action;
    action.start = {ob.cursor.cy, ob.cursor.cx};
    action.removedText = removed;
    action.cursorBefore = before;
    action.cursorAfter = {ob.cursor.cy, ob.cursor.cx};
    action.kind = EditKind::DeleteCharBackward;
    recordAction(action);
}

void Editor::deleteCharForward() {
    auto& ob = bufferManager_.active();

    if (ob.selection.isActive && !ob.selection.empty()) {
        deleteSelection();
        return;
    }

    if (ob.cursor.cy >= ob.buffer.numRows()) return;
    bool atLineEnd = ob.cursor.cx == ob.buffer.row(ob.cursor.cy).size();
    if (atLineEnd && ob.cursor.cy == ob.buffer.numRows() - 1) return;

    model::Position before{ob.cursor.cy, ob.cursor.cx};
    std::string removed;

    int targetRow = ob.cursor.cy;
    int targetCol = ob.cursor.cx;
    if (!atLineEnd) {
        removed = ob.buffer.row(ob.cursor.cy).substrByChar(ob.cursor.cx, ob.cursor.cx + 1);
        targetCol = ob.cursor.cx + 1;
    } else {
        removed = "\n";
        targetRow = ob.cursor.cy + 1;
        targetCol = 0;
    }

    auto pos = ob.buffer.deleteChar(targetRow, targetCol);
    ob.cursor.cy = pos.row;
    ob.cursor.cx = pos.col;

    if (ob.activeSyntax && ob.cursor.cy < ob.buffer.numRows()) {
        syntax::updateSyntax(ob.buffer, ob.buffer.row(ob.cursor.cy), ob.activeSyntax);
    }

    EditAction action;
    action.start = before;
    action.removedText = removed;
    action.cursorBefore = before;
    action.cursorAfter = {ob.cursor.cy, ob.cursor.cx};
    action.kind = EditKind::DeleteCharForward;
    recordAction(action);
}

void Editor::undo() {
    auto& ob = bufferManager_.active();
    const EditAction* action = ob.undoStack.undo();
    if (!action) {
        setStatusMessage("Rien a annuler");
        return;
    }

    ob.selection.isActive = false;

    if (!action->insertedText.empty()) {
        model::Position to = endPositionAfterText(action->start, action->insertedText);
        deleteSelectionForUndo(action->start, to);
    }

    if (!action->removedText.empty()) {
        model::Position endPos;
        insertTextAt(action->start, action->removedText, &endPos);
    }

    ob.cursor.cy = action->cursorBefore.row;
    ob.cursor.cx = action->cursorBefore.col;

    setStatusMessage("Annule");
}

void Editor::redo() {
    auto& ob = bufferManager_.active();
    const EditAction* action = ob.undoStack.redo();
    if (!action) {
        setStatusMessage("Rien a retablir");
        return;
    }

    ob.selection.isActive = false;

    if (!action->removedText.empty()) {
        model::Position to = endPositionAfterText(action->start, action->removedText);
        deleteSelectionForUndo(action->start, to);
    }

    if (!action->insertedText.empty()) {
        model::Position endPos;
        insertTextAt(action->start, action->insertedText, &endPos);
    }

    ob.cursor.cy = action->cursorAfter.row;
    ob.cursor.cx = action->cursorAfter.col;

    setStatusMessage("Retabli");
}

void Editor::copySelection() {
    auto& ob = bufferManager_.active();
    if (!ob.selection.isActive || ob.selection.empty()) {
        setStatusMessage("Aucune selection a copier");
        return;
    }
    std::string text = textInRange(ob.selection.start(), ob.selection.end());
    bool ok = clipboard_.copy(text);
    setStatusMessage(ok ? "Copie dans le presse-papier systeme" : "Copie (presse-papier interne)");
}

void Editor::cutSelection() {
    auto& ob = bufferManager_.active();
    if (!ob.selection.isActive || ob.selection.empty()) {
        setStatusMessage("Aucune selection a couper");
        return;
    }
    std::string text = textInRange(ob.selection.start(), ob.selection.end());
    clipboard_.copy(text);
    deleteSelection();
    setStatusMessage("Coupe");
}

void Editor::pasteClipboard() {
    auto& ob = bufferManager_.active();
    std::string text = clipboard_.paste();
    if (text.empty()) {
        setStatusMessage("Presse-papier vide");
        return;
    }

    if (ob.selection.isActive && !ob.selection.empty()) {
        deleteSelection();
    }

    model::Position before{ob.cursor.cy, ob.cursor.cx};
    model::Position after;
    insertTextAt(before, text, &after);
    ob.cursor.cy = after.row;
    ob.cursor.cx = after.col;

    EditAction action;
    action.start = before;
    action.insertedText = text;
    action.cursorBefore = before;
    action.cursorAfter = after;
    action.kind = EditKind::InsertBlock;
    recordAction(action);

    setStatusMessage("Colle");
}

void Editor::toggleHelpBar() {
    showHelpBar_ = !showHelpBar_;
}

void Editor::toggleFullDoc() {
    overlay_ = (overlay_ == view::OverlayMode::FullDoc) ? view::OverlayMode::None
                                                          : view::OverlayMode::FullDoc;
}

void Editor::clearFindHighlight() {
    auto& ob = bufferManager_.active();
    for (int r = 0; r < ob.buffer.numRows(); r++) {
        if (ob.activeSyntax) {
            syntax::updateSyntax(ob.buffer, ob.buffer.row(r), ob.activeSyntax);
        } else {
            auto& hl = ob.buffer.row(r).hl();
            std::fill(hl.begin(), hl.end(), model::Highlight::Normal);
        }
    }
}

void Editor::updateFindHighlight() {
    auto& ob = bufferManager_.active();
    searchTotalMatches_ = 0;
    if (searchTerm_.empty()) return;

    int matchContainingCursor = 0;
    for (int r = 0; r < ob.buffer.numRows(); r++) {
        const std::string& render = ob.buffer.row(r).render();
        size_t pos = 0;
        while ((pos = render.find(searchTerm_, pos)) != std::string::npos) {
            searchTotalMatches_++;
            if (r == ob.cursor.cy) {
                int matchCharIdx = model::utf8::charIndexForByteOffset(render, pos);
                if (matchCharIdx <= ob.cursor.cx) {
                    matchContainingCursor = searchTotalMatches_;
                }
            } else if (r < ob.cursor.cy) {
                matchContainingCursor = searchTotalMatches_;
            }
            pos += 1;
        }
    }
    searchCurrentMatch_ = matchContainingCursor;
}

void Editor::findNext(int direction) {
    auto& ob = bufferManager_.active();
    if (searchTerm_.empty()) return;

    int startRow = ob.cursor.cy;
    int startCol = ob.cursor.cx;

    int row = startRow;
    for (int steps = 0; steps <= ob.buffer.numRows(); steps++) {
        const model::Row& r = ob.buffer.row(row);
        const std::string& render = r.render();

        if (direction > 0) {
            int fromCol = (row == startRow) ? startCol + 1 : 0;
            size_t byteFrom = model::utf8::byteOffsetForCharIndex(render, fromCol);
            size_t pos = render.find(searchTerm_, byteFrom);
            if (pos != std::string::npos) {
                ob.cursor.cy = row;
                ob.cursor.cx = r.rxToCx(0) + model::utf8::charIndexForByteOffset(render, pos);
                updateFindHighlight();
                return;
            }
        } else {
            int upto = (row == startRow) ? startCol : r.size() + 1;
            size_t byteUpto = model::utf8::byteOffsetForCharIndex(render, upto);
            size_t lastPos = std::string::npos;
            size_t searchPos = 0;
            while (true) {
                size_t found = render.find(searchTerm_, searchPos);
                if (found == std::string::npos || found >= byteUpto) break;
                lastPos = found;
                searchPos = found + 1;
            }
            if (lastPos != std::string::npos) {
                ob.cursor.cy = row;
                ob.cursor.cx = model::utf8::charIndexForByteOffset(render, lastPos);
                updateFindHighlight();
                return;
            }
        }

        row = direction > 0 ? (row + 1) % ob.buffer.numRows() : (row - 1 + ob.buffer.numRows()) % ob.buffer.numRows();
    }

    setStatusMessage("Aucune occurrence trouvee");
}

void Editor::find() {
    auto& ob = bufferManager_.active();
    searchSavedCursor_ = {ob.cursor.cy, ob.cursor.cx};
    searchSavedRowoff_ = ob.rowoff;
    searchSavedColoff_ = ob.coloff;
    searchActive_ = true;
    searchTerm_.clear();

    std::string result = prompt("Rechercher: %s (Echap pour annuler, Entree/fleches pour naviguer)",
        [this](const std::string& text, int key) {
            searchTerm_ = text;
            if (key == EditorKey::ARROW_DOWN || key == EditorKey::ARROW_RIGHT || key == '\r') {
                findNext(1);
            } else if (key == EditorKey::ARROW_UP || key == EditorKey::ARROW_LEFT) {
                findNext(-1);
            } else {
                updateFindHighlight();
                if (searchTotalMatches_ > 0) findNext(1);
            }
        });

    searchActive_ = false;

    if (result.empty()) {
        auto& ob2 = bufferManager_.active();
        ob2.cursor.cy = searchSavedCursor_.row;
        ob2.cursor.cx = searchSavedCursor_.col;
        ob2.rowoff = searchSavedRowoff_;
        ob2.coloff = searchSavedColoff_;
    }
    clearFindHighlight();
}

void Editor::gotoLine() {
    std::string result = prompt("Aller a la ligne: %s (Echap pour annuler)", nullptr);
    if (result.empty()) return;

    char* endPtr = nullptr;
    long lineNum = std::strtol(result.c_str(), &endPtr, 10);
    if (endPtr == result.c_str() || lineNum <= 0) {
        setStatusMessage("Numero de ligne invalide", view::MessageKind::Error);
        return;
    }

    auto& ob = bufferManager_.active();
    int target = static_cast<int>(lineNum) - 1;
    if (target >= ob.buffer.numRows()) target = ob.buffer.numRows() - 1;
    if (target < 0) target = 0;

    ob.cursor.cy = target;
    ob.cursor.cx = 0;
    ob.selection.isActive = false;
}

void Editor::replace() {
    std::string searchTerm = prompt("Remplacer: cherche %s (Echap pour annuler)", nullptr);
    if (searchTerm.empty()) return;

    std::string replaceWith = prompt("Remplacer par: %s (Echap pour annuler)", nullptr);

    auto& ob = bufferManager_.active();
    int replacedCount = 0;

    for (int r = 0; r < ob.buffer.numRows(); r++) {
        size_t searchFromByte = 0;
        while (true) {
            const std::string& render = ob.buffer.row(r).render();
            size_t pos = render.find(searchTerm, searchFromByte);
            if (pos == std::string::npos) break;

            int charIdx = model::utf8::charIndexForByteOffset(render, pos);
            ob.cursor.cy = r;
            ob.cursor.cx = charIdx;
            searchActive_ = true;
            searchTerm_ = searchTerm;
            updateFindHighlight();
            refreshScreen();

            setStatusMessage("Remplacer cette occurrence ? (o)ui / (n)on / (t)out / Echap pour arreter");
            refreshScreen();
            int key = terminal_.readKey();
            searchActive_ = false;

            if (key == '\x1b') {
                clearFindHighlight();
                setStatusMessage("Remplacement interrompu");
                return;
            }

            bool doReplace = (key == 'o' || key == 'O');
            bool doAll = (key == 't' || key == 'T');

            if (doReplace || doAll) {
                int charCount = model::utf8::codepointCount(searchTerm);
                model::Position start{r, charIdx};
                for (int i = 0; i < charCount; i++) {
                    ob.buffer.deleteChar(r, charIdx + 1);
                }
                model::Position endPos;
                insertTextAt(start, replaceWith, &endPos);

                EditAction action;
                action.start = start;
                action.removedText = searchTerm;
                action.insertedText = replaceWith;
                action.cursorBefore = start;
                action.cursorAfter = endPos;
                action.kind = EditKind::ReplaceBlock;
                recordAction(action);

                replacedCount++;
                if (doAll) {
                    ob.cursor.cx = endPos.col;
                    searchFromByte = model::utf8::byteOffsetForCharIndex(ob.buffer.row(r).render(), endPos.col);
                    continue;
                }
                ob.cursor.cx = endPos.col;
                break;
            }
            searchFromByte = pos + 1;
            ob.cursor.cx = charIdx + 1;
        }
    }

    clearFindHighlight();
    char msg[64];
    snprintf(msg, sizeof(msg), "%d remplacement(s) effectue(s)", replacedCount);
    setStatusMessage(msg, view::MessageKind::Success);
}

void Editor::handleMouseEvent(const terminal::MouseEvent& ev) {
    auto& ob = bufferManager_.active();

    if (ev.isWheel) {
        int textRows = renderer_.textAreaScreenRows(view::ViewState{
            bufferManager_, ob.rowoff, ob.coloff, screenrows_, screencols_,
            statusMessage_, statusMessageKind_, 0, overlay_, showHelpBar_, {}, config_});
        int delta = ev.wheelDelta * 3;
        ob.rowoff += delta;
        if (ob.rowoff < 0) ob.rowoff = 0;
        int maxOff = std::max(ob.buffer.numRows() - textRows, 0);
        if (ob.rowoff > maxOff) ob.rowoff = maxOff;
        return;
    }

    if (!ev.pressed) return;

    int tabBarHeight = (bufferManager_.count() > 1) ? 1 : 0;
    int gutter = renderer_.gutterWidth(view::ViewState{
        bufferManager_, ob.rowoff, ob.coloff, screenrows_, screencols_,
        statusMessage_, statusMessageKind_, 0, overlay_, showHelpBar_, {}, config_});

    int screenRow = ev.y - tabBarHeight;
    int screenCol = ev.x - gutter;
    if (screenRow < 0 || screenCol < 0) return;

    int filerow = screenRow + ob.rowoff;
    if (filerow >= ob.buffer.numRows()) {
        filerow = ob.buffer.numRows() > 0 ? ob.buffer.numRows() - 1 : 0;
    }
    if (filerow < 0) return;

    int rx = screenCol + ob.coloff;
    int cx = ob.buffer.numRows() > 0 ? ob.buffer.row(filerow).rxToCx(rx) : 0;

    ob.cursor.cy = filerow;
    ob.cursor.cx = cx;
    ob.selection.isActive = false;
}

void Editor::processKeypress() {
    int c = terminal_.readKey();
    auto& ob = bufferManager_.active();

    if (overlay_ == view::OverlayMode::FullDoc) {
        if (c == '\x1b' || c == ctrl('d')) {
            overlay_ = view::OverlayMode::None;
        }
        return;
    }

    if (c == EditorKey::MOUSE_EVENT) {
        handleMouseEvent(terminal_.lastMouseEvent());
        return;
    }

    switch (c) {
        case '\r':
            insertNewline();
            break;

        case ctrl('q'):
            if (bufferManager_.anyDirty() && quitTimes_ > 0) {
                char msg[128];
                snprintf(msg, sizeof(msg),
                         "ATTENTION ! Des modifications ne sont pas sauvegardees. "
                         "Appuyez sur Ctrl-Q %d fois de plus pour quitter.",
                         quitTimes_);
                setStatusMessage(msg, view::MessageKind::Error);
                quitTimes_--;
                return;
            }
            terminal_.write("\x1b[?1049l", 8);
            std::exit(0);

        case ctrl('s'):
            save();
            break;

        case ctrl('f'):
            find();
            break;

        case ctrl('r'):
            replace();
            break;

        case ctrl('z'):
            undo();
            break;

        case ctrl('y'):
            redo();
            break;

        case ctrl('x'):
            cutSelection();
            break;

        case ctrl('c'):
            copySelection();
            break;

        case ctrl('v'):
            pasteClipboard();
            break;

        case ctrl('l'):
            gotoLine();
            break;

        case ctrl('n'):
            bufferManager_.next();
            break;

        case ctrl('p'):
            bufferManager_.previous();
            break;

        case ctrl('h'):
            toggleHelpBar();
            break;

        case ctrl('d'):
            toggleFullDoc();
            break;

        case EditorKey::HOME_KEY:
            ob.cursor.cx = 0;
            ob.selection.isActive = false;
            break;

        case EditorKey::END_KEY:
            if (ob.cursor.cy < ob.buffer.numRows()) {
                ob.cursor.cx = ob.buffer.row(ob.cursor.cy).size();
            }
            ob.selection.isActive = false;
            break;

        case EditorKey::BACKSPACE:
        case EditorKey::DEL_KEY:
            if (c == EditorKey::DEL_KEY) {
                deleteCharForward();
            } else {
                deleteCharBackward();
            }
            break;

        case EditorKey::PAGE_UP:
        case EditorKey::PAGE_DOWN: {
            int textRows = renderer_.textAreaScreenRows(view::ViewState{
                bufferManager_, ob.rowoff, ob.coloff, screenrows_, screencols_,
                statusMessage_, statusMessageKind_, 0, overlay_, showHelpBar_, {}, config_});
            if (c == EditorKey::PAGE_UP) {
                ob.cursor.cy = ob.rowoff;
            } else {
                ob.cursor.cy = ob.rowoff + textRows - 1;
                if (ob.cursor.cy > ob.buffer.numRows()) ob.cursor.cy = ob.buffer.numRows();
            }
            int times = textRows;
            while (times--) {
                moveCursor(c == EditorKey::PAGE_UP ? EditorKey::ARROW_UP : EditorKey::ARROW_DOWN, false);
            }
            break;
        }

        case EditorKey::ARROW_UP:
        case EditorKey::ARROW_DOWN:
        case EditorKey::ARROW_LEFT:
        case EditorKey::ARROW_RIGHT:
            moveCursor(c, false);
            break;

        case EditorKey::CTRL_ARROW_UP:
            moveCursor(EditorKey::ARROW_UP, true);
            break;
        case EditorKey::CTRL_ARROW_DOWN:
            moveCursor(EditorKey::ARROW_DOWN, true);
            break;
        case EditorKey::CTRL_ARROW_LEFT:
            moveCursor(EditorKey::ARROW_LEFT, true);
            break;
        case EditorKey::CTRL_ARROW_RIGHT:
            moveCursor(EditorKey::ARROW_RIGHT, true);
            break;

        case '\x1b':
            ob.selection.isActive = false;
            break;

        default:
            if (terminal::isEncodedCodepoint(c)) {
                insertCodepoint(terminal::decodeCodepoint(c));
            } else if (!std::iscntrl(c) && c < 128) {
                insertCodepoint(static_cast<uint32_t>(c));
            } else if (c >= 1000 && c < 1000 + 20) {

            }
            break;
    }

    quitTimes_ = config_.quitTimes;
}

}
