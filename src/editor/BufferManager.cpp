#include "editor/BufferManager.hpp"

#include "io/FileIO.hpp"

namespace cera::editor {

BufferManager::BufferManager() {
    openNewEmpty();
}

int BufferManager::openNewEmpty() {
    buffers_.emplace_back();
    activeIndex_ = static_cast<int>(buffers_.size()) - 1;
    return activeIndex_;
}

int BufferManager::openFile(const std::string& filename) {
    if (buffers_.size() == 1 && buffers_[0].buffer.numRows() == 0 &&
        !buffers_[0].buffer.filename() && !buffers_[0].buffer.dirty()) {
        io::openFile(buffers_[0].buffer, filename);
        buffers_[0].activeSyntax = syntax::selectForFilename(filename);
        if (buffers_[0].activeSyntax) {
            for (int i = 0; i < buffers_[0].buffer.numRows(); i++) {
                syntax::updateSyntax(buffers_[0].buffer, buffers_[0].buffer.row(i),
                                      buffers_[0].activeSyntax);
            }
        }
        activeIndex_ = 0;
        return 0;
    }

    buffers_.emplace_back();
    int idx = static_cast<int>(buffers_.size()) - 1;
    io::openFile(buffers_[static_cast<size_t>(idx)].buffer, filename);
    buffers_[static_cast<size_t>(idx)].activeSyntax = syntax::selectForFilename(filename);
    if (buffers_[static_cast<size_t>(idx)].activeSyntax) {
        auto& buf = buffers_[static_cast<size_t>(idx)].buffer;
        for (int i = 0; i < buf.numRows(); i++) {
            syntax::updateSyntax(buf, buf.row(i), buffers_[static_cast<size_t>(idx)].activeSyntax);
        }
    }
    activeIndex_ = idx;
    return idx;
}

void BufferManager::closeActive() {
    if (buffers_.size() <= 1) {
        buffers_[0] = OpenBuffer{};
        activeIndex_ = 0;
        return;
    }
    buffers_.erase(buffers_.begin() + activeIndex_);
    if (activeIndex_ >= static_cast<int>(buffers_.size())) {
        activeIndex_ = static_cast<int>(buffers_.size()) - 1;
    }
}

void BufferManager::next() {
    if (buffers_.empty()) return;
    activeIndex_ = (activeIndex_ + 1) % static_cast<int>(buffers_.size());
}

void BufferManager::previous() {
    if (buffers_.empty()) return;
    activeIndex_ = (activeIndex_ - 1 + static_cast<int>(buffers_.size())) % static_cast<int>(buffers_.size());
}

void BufferManager::switchTo(int index) {
    if (index >= 0 && index < static_cast<int>(buffers_.size())) {
        activeIndex_ = index;
    }
}

bool BufferManager::anyDirty() const {
    for (const auto& b : buffers_) {
        if (b.buffer.dirty()) return true;
    }
    return false;
}

}
