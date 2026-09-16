use super::{HighlightFlags, Syntax};

pub fn css_syntax() -> Syntax {
    Syntax {
        filetype: "css",
        filematch: &[".css"],
        keywords: &[],
        singleline_comment_start: "",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
