#include "model/Buffer.hpp"

namespace cera::model {

void Buffer::reindex(int from) {
    for (int j = from; j < numRows(); j++) {
        rows_[static_cast<size_t>(j)].idx = j;
    }
}

void Buffer::insertRow(int at, const std::string& s) {
    if (at < 0 || at > numRows()) return;

    rows_.insert(rows_.begin() + at, Row(s));
    reindex(at);
    dirty_++;
}

void Buffer::deleteRow(int at) {
    if (at < 0 || at >= numRows()) return;

    rows_.erase(rows_.begin() + at);
    reindex(at);
    dirty_++;
}

void Buffer::insertChar(int rowIdx, int col, int c) {
    if (rowIdx == numRows()) {
        insertRow(numRows(), "");
    }
    rows_[static_cast<size_t>(rowIdx)].insertChar(col, static_cast<char>(c));
    dirty_++;
}

void Buffer::insertNewline(int rowIdx, int col) {
    if (col == 0) {
        insertRow(rowIdx, "");
    } else {
        Row& current = rows_[static_cast<size_t>(rowIdx)];
        std::string tail = current.chars().substr(static_cast<size_t>(col));
        std::string head = current.chars().substr(0, static_cast<size_t>(col));
        insertRow(rowIdx + 1, tail);
        rows_[static_cast<size_t>(rowIdx)].setChars(head);
    }
    dirty_++;
}

Buffer::CursorPos Buffer::deleteChar(int rowIdx, int col) {
    if (rowIdx == numRows()) return {rowIdx, col};
    if (col == 0 && rowIdx == 0) return {rowIdx, col};

    Row& row = rows_[static_cast<size_t>(rowIdx)];
    if (col > 0) {
        row.delChar(col - 1);
        dirty_++;
        return {rowIdx, col - 1};
    }

    Row& prev = rows_[static_cast<size_t>(rowIdx - 1)];
    int newCol = prev.size();
    prev.appendString(row.chars());
    deleteRow(rowIdx);
    dirty_++;
    return {rowIdx - 1, newCol};
}

std::string Buffer::toString() const {
    std::string result;
    for (const auto& r : rows_) {
        result += r.chars();
        result += '\n';
    }
    return result;
}

void Buffer::setLines(const std::vector<std::string>& lines) {
    rows_.clear();
    rows_.reserve(lines.size());
    for (const auto& l : lines) {
        rows_.emplace_back(l);
    }
    reindex(0);
    dirty_ = 0;
}

}
