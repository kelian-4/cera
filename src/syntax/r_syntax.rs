use super::{HighlightFlags, Syntax};

pub fn r_syntax() -> Syntax {
    Syntax {
        filetype: "r",
        filematch: &[".r", ".R"],
        keywords: &[
            "function", "if", "else", "for", "while", "repeat", "break", "next", "return",
            "library", "require", "in", "TRUE|", "FALSE|", "NULL|", "NA|", "Inf|", "NaN|",
        ],
        singleline_comment_start: "#",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
