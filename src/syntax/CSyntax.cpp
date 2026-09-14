#include "syntax/Syntax.hpp"

namespace cera::syntax {

const Syntax& cSyntax() {
    static const Syntax kCSyntax{
        "c",
        {".c", ".h", ".cpp", ".hpp", ".cc", ".cxx"},
        {
            "switch", "if", "while", "for", "break", "continue", "return", "else",
            "struct", "union", "typedef", "static", "enum", "class", "case",
            "namespace", "template", "public", "private", "protected", "virtual",
            "override", "const", "constexpr", "using", "new", "delete", "try",
            "catch", "throw",
            "int|", "long|", "double|", "float|", "char|", "unsigned|", "signed|",
            "void|", "bool|", "auto|", "size_t|",
        },
        "//",
        "/*",
        "*/",
        HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS,
    };
    return kCSyntax;
}

}
