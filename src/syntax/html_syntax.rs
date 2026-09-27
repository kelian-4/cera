use super::{HighlightFlags, Syntax};

pub fn html_syntax() -> Syntax {
    Syntax {
        filetype: "html",
        filematch: &[".html", ".htm"],
        keywords: &[],
        singleline_comment_start: "",
        multiline_comment_start: "<!--",
        multiline_comment_end: "-->",
        flags: HighlightFlags::STRINGS,
    }
}
