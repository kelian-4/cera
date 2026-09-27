use super::{HighlightFlags, Syntax};

pub fn yaml_syntax() -> Syntax {
    Syntax {
        filetype: "yaml",
        filematch: &[".yaml", ".yml"],
        keywords: &["true|", "false|", "null|", "yes|", "no|"],
        singleline_comment_start: "#",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
