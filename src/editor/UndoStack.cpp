#include "editor/UndoStack.hpp"

namespace cera::editor {

bool UndoStack::canCoalesce(const EditAction& next) const {
    if (coalescingBroken_) return false;
    if (undoList_.empty()) return false;

    const EditAction& last = undoList_.back();

    bool sameKind = (last.kind == next.kind);
    bool simpleInsert = (next.kind == EditKind::InsertChar &&
                         last.kind == EditKind::InsertChar &&
                         last.removedText.empty() && next.removedText.empty());
    bool simpleBackspace = (next.kind == EditKind::DeleteCharBackward &&
                            last.kind == EditKind::DeleteCharBackward &&
                            last.insertedText.empty() && next.insertedText.empty());
    bool simpleForwardDelete = (next.kind == EditKind::DeleteCharForward &&
                                last.kind == EditKind::DeleteCharForward &&
                                last.insertedText.empty() && next.insertedText.empty());

    if (!sameKind) return false;
    if (!(simpleInsert || simpleBackspace || simpleForwardDelete)) return false;

    return last.cursorAfter == next.start ||
           (next.kind == EditKind::DeleteCharBackward && next.start == last.start);
}

void UndoStack::push(const EditAction& action) {
    redoList_.clear();

    if (canCoalesce(action)) {
        EditAction& last = undoList_.back();
        if (action.kind == EditKind::InsertChar) {
            last.insertedText += action.insertedText;
        } else if (action.kind == EditKind::DeleteCharBackward) {
            last.removedText = action.removedText + last.removedText;
            last.start = action.start;
        } else if (action.kind == EditKind::DeleteCharForward) {
            last.removedText += action.removedText;
        }
        last.cursorAfter = action.cursorAfter;
        return;
    }

    undoList_.push_back(action);
    coalescingBroken_ = false;
}

void UndoStack::breakCoalescing() {
    coalescingBroken_ = true;
}

const EditAction* UndoStack::undo() {
    if (undoList_.empty()) return nullptr;
    redoList_.push_back(undoList_.back());
    undoList_.pop_back();
    coalescingBroken_ = true;
    return &redoList_.back();
}

const EditAction* UndoStack::redo() {
    if (redoList_.empty()) return nullptr;
    undoList_.push_back(redoList_.back());
    redoList_.pop_back();
    coalescingBroken_ = true;
    return &undoList_.back();
}

void UndoStack::clear() {
    undoList_.clear();
    redoList_.clear();
    coalescingBroken_ = true;
}

}
