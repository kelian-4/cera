#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "model/Buffer.hpp"
#include "model/Row.hpp"

namespace cera::syntax {

enum HighlightFlags : uint32_t {
    HL_HIGHLIGHT_NUMBERS = 1 << 0,
    HL_HIGHLIGHT_STRINGS = 1 << 1,
};

struct Syntax {
    std::string filetype;
    std::vector<std::string> filematch;

    std::vector<std::string> keywords;
    std::string singlelineCommentStart;
    std::string multilineCommentStart;
    std::string multilineCommentEnd;
    uint32_t flags = 0;
};

bool matches(const Syntax& syntax, const std::string& filename);

void updateSyntax(model::Buffer& buffer, model::Row& row, const Syntax* syntax);

const std::vector<Syntax>& database();

const Syntax* selectForFilename(const std::string& filename);

}
