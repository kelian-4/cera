use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub fn yaml_syntax() -> Syntax {
    Syntax {
        filetype: "yaml",
        filematch: &[".yaml", ".yml"],
        keywords: &[],
        types: &[],
        constants: &[
            "true", "false", "null", "yes", "no", "on", "off", "True", "False", "Null", "~",
        ],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &["!!"],
        variable_prefixes: "&*",
        ident_chars: "",
        operators: "",
        line_rules: &[
            ("---", Highlight::Preprocessor),
            ("...", Highlight::Preprocessor),
        ],
        aliases: &["yml"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::KEY_VALUES)
            .union(HighlightFlags::KEY_STRINGS),
    }
}
