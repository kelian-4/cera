use super::{HighlightFlags, Syntax};

pub fn c_syntax() -> Syntax {
    Syntax {
        filetype: "c",
        filematch: &[".c", ".h", ".cpp", ".hpp", ".cc", ".cxx"],
        keywords: &[
            "switch", "if", "while", "for", "break", "continue", "return", "else",
            "struct", "union", "typedef", "static", "enum", "class", "case",
            "namespace", "template", "public", "private", "protected", "virtual",
            "override", "const", "constexpr", "using", "new", "delete", "try",
            "catch", "throw",
            "int|", "long|", "double|", "float|", "char|", "unsigned|", "signed|",
            "void|", "bool|", "auto|", "size_t|",
        ],
        singleline_comment_start: "//",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
