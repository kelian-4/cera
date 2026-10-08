use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub fn ini_syntax() -> Syntax {
    Syntax {
        filetype: "ini",
        filematch: &[
            ".ini",
            ".conf",
            ".cfg",
            ".service",
            ".timer",
            ".socket",
            ".mount",
            ".desktop",
            ".properties",
            ".env",
            ".editorconfig",
            ".gitconfig",
        ],
        keywords: &[],
        types: &[],
        constants: &["true", "false", "yes", "no", "on", "off"],
        singleline_comment_start: ";",
        alt_comment_start: "#",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[("[", Highlight::Keyword1)],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::KEY_VALUES),
    }
}
