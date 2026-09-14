#pragma once

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "model/Row.hpp"

namespace cera::model {

class Buffer {
public:
    Buffer() = default;

    int numRows() const { return static_cast<int>(rows_.size()); }
    bool empty() const { return rows_.empty(); }

    const Row& row(int i) const { return rows_[static_cast<size_t>(i)]; }
    Row& row(int i) { return rows_[static_cast<size_t>(i)]; }

    const std::vector<Row>& rows() const { return rows_; }

    void insertRow(int at, const std::string& s);

    void deleteRow(int at);

    void insertChar(int row, int col, uint32_t codepoint);

    void insertNewline(int row, int col);

    struct CursorPos { int row; int col; };
    CursorPos deleteChar(int row, int col);

    std::string toString() const;

    void setLines(const std::vector<std::string>& lines);

    bool dirty() const { return dirty_ > 0; }
    void clearDirty() { dirty_ = 0; }
    void markDirty() { dirty_++; }

    const std::optional<std::string>& filename() const { return filename_; }
    void setFilename(std::string name) { filename_ = std::move(name); }

private:
    void reindex(int from);

    std::vector<Row> rows_;
    std::optional<std::string> filename_;
    int dirty_ = 0;
};

}
