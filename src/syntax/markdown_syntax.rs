use super::{HighlightFlags, Syntax};

pub fn markdown_syntax() -> Syntax {
    Syntax {
        filetype: "markdown",
        filematch: &[".md", ".markdown"],
        keywords: &[],
        singleline_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::empty(),
    }
}
