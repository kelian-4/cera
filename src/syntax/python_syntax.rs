use super::{HighlightFlags, Syntax};

pub fn python_syntax() -> Syntax {
    Syntax {
        filetype: "python",
        filematch: &[".py", ".pyw"],
        keywords: &[
            "def", "class", "return", "if", "elif", "else", "while", "for", "break", "continue",
            "pass", "import", "from", "as", "with", "try", "except", "finally", "raise", "lambda",
            "yield", "global", "nonlocal", "assert", "del", "in", "is", "not", "and", "or",
            "None|", "True|", "False|", "self|", "int|", "str|", "float|", "bool|", "list|",
            "dict|", "tuple|", "set|",
        ],
        singleline_comment_start: "#",
        multiline_comment_start: "\"\"\"",
        multiline_comment_end: "\"\"\"",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
