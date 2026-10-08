use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub fn toml_syntax() -> Syntax {
    Syntax {
        filetype: "toml",
        filematch: &[".toml"],
        keywords: &[],
        types: &[],
        constants: &["true", "false", "inf", "nan"],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[("\"\"\"", "\"\"\""), ("'''", "'''")],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[("[", Highlight::Keyword1)],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::KEY_VALUES)
            .union(HighlightFlags::KEY_STRINGS),
    }
}
