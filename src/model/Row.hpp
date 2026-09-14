#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace cera::model {

enum class Highlight : unsigned char {
    Normal = 0,
    Comment,
    MLComment,
    Keyword1,
    Keyword2,
    String,
    Number,
    Match
};

class Row {
public:
    Row() = default;
    explicit Row(std::string chars);

    const std::string& chars() const { return chars_; }
    void setChars(std::string chars);

    const std::string& render() const { return render_; }
    const std::vector<Highlight>& hl() const { return hl_; }
    std::vector<Highlight>& hl() { return hl_; }

    int size() const { return charCount_; }
    int byteSize() const { return static_cast<int>(chars_.size()); }
    int rsize() const { return renderCharCount_; }

    const std::vector<size_t>& renderByteOffsets() const { return renderByteOffsets_; }
    int renderCharWidth(int renderCharIdx) const;

    void insertChar(int at, uint32_t codepoint);

    void delChar(int at);

    void appendString(const std::string& s);

    std::string substrByChar(int fromCharIdx, int toCharIdx) const;

    int cxToRx(int cx) const;

    int rxToCx(int rx) const;

    void updateRender();

    bool hlOpenComment = false;

    int idx = 0;

    static int tabStop;

private:
    size_t byteOffsetOf(int charIdx) const;

    std::string chars_;
    int charCount_ = 0;

    std::string render_;
    int renderCharCount_ = 0;
    std::vector<size_t> renderByteOffsets_;

    std::vector<Highlight> hl_;
};

}
