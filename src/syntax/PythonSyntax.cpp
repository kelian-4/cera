#include "syntax/Syntax.hpp"

namespace cera::syntax {

const Syntax& pythonSyntax() {
    static const Syntax kPythonSyntax{
        "python",
        {".py", ".pyw"},
        {
            "def", "class", "return", "if", "elif", "else", "while", "for",
            "break", "continue", "pass", "import", "from", "as", "with",
            "try", "except", "finally", "raise", "lambda", "yield", "global",
            "nonlocal", "assert", "del", "in", "is", "not", "and", "or",
            "None|", "True|", "False|", "self|", "int|", "str|", "float|",
            "bool|", "list|", "dict|", "tuple|", "set|",
        },
        "#",
        "\"\"\"",
        "\"\"\"",
        HL_HIGHLIGHT_NUMBERS | HL_HIGHLIGHT_STRINGS,
    };
    return kPythonSyntax;
}

}
