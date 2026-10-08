use super::{HighlightFlags, Syntax};

pub fn html_syntax() -> Syntax {
    Syntax {
        filetype: "html",
        filematch: &[".html", ".htm"],
        keywords: &[],
        types: &[],
        constants: &[],
        singleline_comment_start: "",
        alt_comment_start: "",
        multiline_comment_start: "<!--",
        multiline_comment_end: "-->",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        flags: HighlightFlags::STRINGS.union(HighlightFlags::MARKUP),
    }
}
