#pragma once

#include <cstddef>
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

    int size() const { return static_cast<int>(chars_.size()); }
    int rsize() const { return static_cast<int>(render_.size()); }

    void insertChar(int at, char c);

    void delChar(int at);

    void appendString(const std::string& s);

    int cxToRx(int cx) const;

    int rxToCx(int rx) const;

    void updateRender();

    bool hlOpenComment = false;

    int idx = 0;

    static constexpr int kTabStop = 8;

private:
    std::string chars_;
    std::string render_;
    std::vector<Highlight> hl_;
};

}
