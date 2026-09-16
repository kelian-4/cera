use super::{HighlightFlags, Syntax};

pub fn json_syntax() -> Syntax {
    Syntax {
        filetype: "json",
        filematch: &[".json"],
        keywords: &["true|", "false|", "null|"],
        singleline_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
