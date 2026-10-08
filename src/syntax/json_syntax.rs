use super::{HighlightFlags, Syntax};

pub fn json_syntax() -> Syntax {
    Syntax {
        filetype: "json",
        filematch: &[".json"],
        keywords: &[],
        types: &[],
        constants: &["true", "false", "null"],
        singleline_comment_start: "//",
        alt_comment_start: "",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        multiline_strings: &[],
        quotes: "\"",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::KEY_STRINGS),
    }
}
