#include "syntax/Syntax.hpp"

namespace cera::syntax {

const Syntax& javascriptSyntax() {
    static const Syntax kJsSyntax{
        "javascript",
        {".js", ".jsx", ".ts", ".tsx", ".mjs"},
        {
            "function", "return", "if", "else", "while", "for", "break",
            "continue", "switch", "case", "default", "do", "try", "catch",
            "finally", "throw", "class", "extends", "super", "new", "delete",
            "typeof", "instanceof", "in", "of", "import", "export", "from",
            "as", "async", "await", "yield", "static", "get", "set",
            "let|", "const|", "var|", "true|", "false|", "null|",
            "undefined|", "this|", "number|", "string|", "boolean|",
        },
        "//",
        "/*",
        "*/",
        HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS,
    };
    return kJsSyntax;
}

}
