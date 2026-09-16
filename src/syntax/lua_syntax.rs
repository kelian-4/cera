use super::{HighlightFlags, Syntax};

pub fn lua_syntax() -> Syntax {
    Syntax {
        filetype: "lua",
        filematch: &[".lua"],
        keywords: &[
            "function", "end", "if", "then", "else", "elseif", "for", "while", "repeat", "until",
            "do", "break", "return", "local", "in", "and", "or", "not", "require", "true|",
            "false|", "nil|",
        ],
        singleline_comment_start: "--",
        multiline_comment_start: "--[[",
        multiline_comment_end: "]]",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
