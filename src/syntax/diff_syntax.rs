use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub fn diff_syntax() -> Syntax {
    Syntax {
        filetype: "diff",
        filematch: &[".diff", ".patch"],
        keywords: &[],
        types: &[],
        constants: &[],
        singleline_comment_start: "",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[
            ("diff ", Highlight::Keyword1),
            ("index ", Highlight::Comment),
            ("--- ", Highlight::Keyword1),
            ("+++ ", Highlight::Keyword1),
            ("@@", Highlight::Preprocessor),
            ("+", Highlight::String),
            ("-", Highlight::Constant),
            ("\\", Highlight::Comment),
        ],
        aliases: &["patch"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::empty(),
    }
}
