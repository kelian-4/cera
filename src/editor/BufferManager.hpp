#pragma once

#include <string>
#include <vector>

#include "editor/UndoStack.hpp"
#include "model/Buffer.hpp"
#include "model/Cursor.hpp"
#include "model/Selection.hpp"
#include "syntax/Syntax.hpp"

namespace cera::editor {

struct OpenBuffer {
    model::Buffer buffer;
    model::Cursor cursor;
    model::Selection selection;
    int rowoff = 0;
    int coloff = 0;
    const syntax::Syntax* activeSyntax = nullptr;
    UndoStack undoStack;
};

class BufferManager {
public:
    BufferManager();

    int count() const { return static_cast<int>(buffers_.size()); }
    int activeIndex() const { return activeIndex_; }

    OpenBuffer& active() { return buffers_[static_cast<size_t>(activeIndex_)]; }
    const OpenBuffer& active() const { return buffers_[static_cast<size_t>(activeIndex_)]; }

    OpenBuffer& at(int index) { return buffers_[static_cast<size_t>(index)]; }
    const OpenBuffer& at(int index) const { return buffers_[static_cast<size_t>(index)]; }

    int openNewEmpty();

    int openFile(const std::string& filename);

    void closeActive();

    void next();
    void previous();
    void switchTo(int index);

    bool anyDirty() const;

private:
    std::vector<OpenBuffer> buffers_;
    int activeIndex_ = 0;
};

}
