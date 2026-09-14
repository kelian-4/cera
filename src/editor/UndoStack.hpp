#pragma once

#include <string>
#include <vector>

#include "model/Buffer.hpp"
#include "model/Selection.hpp"

namespace cera::editor {

enum class EditKind {
    InsertChar,
    DeleteCharBackward,
    DeleteCharForward,
    InsertNewline,
    InsertBlock,
    DeleteBlock,
    ReplaceBlock,
    Other
};

struct EditAction {
    model::Position start;
    std::string removedText;
    std::string insertedText;
    model::Position cursorBefore;
    model::Position cursorAfter;
    EditKind kind = EditKind::Other;
};

class UndoStack {
public:
    void push(const EditAction& action);

    void breakCoalescing();

    bool canUndo() const { return !undoList_.empty(); }
    bool canRedo() const { return !redoList_.empty(); }

    const EditAction* undo();
    const EditAction* redo();

    void clear();

private:
    bool canCoalesce(const EditAction& next) const;

    std::vector<EditAction> undoList_;
    std::vector<EditAction> redoList_;
    bool coalescingBroken_ = true;
};

}
