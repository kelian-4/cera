#include "syntax/Syntax.hpp"

#include <cctype>
#include <cstring>

#include "model/Utf8.hpp"

namespace cera::syntax {

namespace {

bool isSeparator(int c) {
    return std::isspace(c) || c == '\0' ||
           std::strchr(",.()+-/*=~%<>[];", c) != nullptr;
}

bool isKeyword2(const std::string& kw) {
    return !kw.empty() && kw.back() == '|';
}

}

bool matches(const Syntax& syntax, const std::string& filename) {
    size_t dot = filename.find_last_of('.');
    std::string ext = (dot == std::string::npos) ? "" : filename.substr(dot);

    for (const auto& pattern : syntax.filematch) {
        bool isExt = !pattern.empty() && pattern[0] == '.';
        if (isExt && !ext.empty() && ext == pattern) return true;
        if (!isExt && filename.find(pattern) != std::string::npos) return true;
    }
    return false;
}

void updateSyntax(model::Buffer& buffer, model::Row& row, const Syntax* syntax) {
    auto& hl = row.hl();
    hl.assign(static_cast<size_t>(row.rsize()), model::Highlight::Normal);

    if (syntax == nullptr) return;

    const std::string& render = row.render();
    std::vector<model::Highlight> hlByte(render.size(), model::Highlight::Normal);

    const std::string& scs = syntax->singlelineCommentStart;
    const std::string& mcs = syntax->multilineCommentStart;
    const std::string& mce = syntax->multilineCommentEnd;

    bool prevSep = true;
    char inString = '\0';
    bool inComment = (row.idx > 0 && buffer.row(row.idx - 1).hlOpenComment);

    int rsize = static_cast<int>(render.size());
    int i = 0;

    while (i < rsize) {
        char c = render[static_cast<size_t>(i)];
        model::Highlight prevHl =
            (i > 0) ? hlByte[static_cast<size_t>(i - 1)] : model::Highlight::Normal;

        if (!scs.empty() && !inString && !inComment) {
            if (render.compare(static_cast<size_t>(i), scs.size(), scs) == 0) {
                for (int k = i; k < rsize; k++) hlByte[static_cast<size_t>(k)] = model::Highlight::Comment;
                break;
            }
        }

        if (!mcs.empty() && !mce.empty() && !inString) {
            if (inComment) {
                hlByte[static_cast<size_t>(i)] = model::Highlight::MLComment;
                if (render.compare(static_cast<size_t>(i), mce.size(), mce) == 0) {
                    for (size_t k = 0; k < mce.size(); k++) {
                        hlByte[static_cast<size_t>(i) + k] = model::Highlight::MLComment;
                    }
                    i += static_cast<int>(mce.size());
                    inComment = false;
                    prevSep = true;
                    continue;
                }
                i++;
                continue;
            }
            if (render.compare(static_cast<size_t>(i), mcs.size(), mcs) == 0) {
                for (size_t k = 0; k < mcs.size(); k++) {
                    hlByte[static_cast<size_t>(i) + k] = model::Highlight::MLComment;
                }
                i += static_cast<int>(mcs.size());
                inComment = true;
                continue;
            }
        }

        if (syntax->flags & HL_HIGHLIGHT_STRINGS) {
            if (inString) {
                hlByte[static_cast<size_t>(i)] = model::Highlight::String;
                if (c == '\\' && i + 1 < rsize) {
                    hlByte[static_cast<size_t>(i + 1)] = model::Highlight::String;
                    i += 2;
                    continue;
                }
                if (c == inString) inString = '\0';
                i++;
                prevSep = true;
                continue;
            }
            if (c == '"' || c == '\'') {
                inString = c;
                hlByte[static_cast<size_t>(i)] = model::Highlight::String;
                i++;
                continue;
            }
        }

        if (syntax->flags & HL_HIGHLIGHT_NUMBERS) {
            if ((std::isdigit(static_cast<unsigned char>(c)) &&
                 (prevSep || prevHl == model::Highlight::Number)) ||
                (c == '.' && prevHl == model::Highlight::Number)) {
                hlByte[static_cast<size_t>(i)] = model::Highlight::Number;
                i++;
                prevSep = false;
                continue;
            }
        }

        if (prevSep) {
            bool matched = false;
            for (const auto& kw : syntax->keywords) {
                bool kw2 = isKeyword2(kw);
                size_t klen = kw2 ? kw.size() - 1 : kw.size();

                if (render.compare(static_cast<size_t>(i), klen, kw, 0, klen) == 0 &&
                    isSeparator(static_cast<unsigned char>(
                        (static_cast<size_t>(i) + klen < render.size())
                            ? render[static_cast<size_t>(i) + klen]
                            : '\0'))) {
                    model::Highlight h =
                        kw2 ? model::Highlight::Keyword2 : model::Highlight::Keyword1;
                    for (size_t k = 0; k < klen; k++) {
                        hlByte[static_cast<size_t>(i) + k] = h;
                    }
                    i += static_cast<int>(klen);
                    matched = true;
                    break;
                }
            }
            if (matched) {
                prevSep = false;
                continue;
            }
        }

        prevSep = isSeparator(static_cast<unsigned char>(c));
        i++;
    }

    const auto& offsets = row.renderByteOffsets();
    for (int charIdx = 0; charIdx < row.rsize(); charIdx++) {
        size_t byteOff = offsets[static_cast<size_t>(charIdx)];
        hl[static_cast<size_t>(charIdx)] = hlByte[byteOff];
    }

    bool changed = (row.hlOpenComment != inComment);
    row.hlOpenComment = inComment;
    if (changed && row.idx + 1 < buffer.numRows()) {
        updateSyntax(buffer, buffer.row(row.idx + 1), syntax);
    }
}

const Syntax* selectForFilename(const std::string& filename) {
    for (const auto& s : database()) {
        if (matches(s, filename)) return &s;
    }
    return nullptr;
}

const std::vector<Syntax>& database() {
    static const std::vector<Syntax> db = {
        cSyntax(),
        pythonSyntax(),
        javascriptSyntax(),
        markdownSyntax(),
    };
    return db;
}

}
