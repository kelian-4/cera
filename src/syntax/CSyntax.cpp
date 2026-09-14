#include "syntax/Syntax.hpp"

namespace cera::syntax {

namespace {

const Syntax kCSyntax{
     "c",
     {".c", ".h", ".cpp"},

    {
        "switch", "if", "while", "for", "break", "continue", "return", "else",
        "struct", "union", "typedef", "static", "enum", "class", "case",
        "int|", "long|", "double|", "float|", "char|", "unsigned|", "signed|",
        "void|",
    },
     "//",
     "/*",
     "*/",
     HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS,
};

}

const std::vector<Syntax>& database() {
    static const std::vector<Syntax> db = {kCSyntax};
    return db;
}

}
