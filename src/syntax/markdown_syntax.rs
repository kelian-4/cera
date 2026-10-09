use super::{HighlightFlags, Syntax};
use crate::model::row::Highlight;

pub fn markdown_syntax() -> Syntax {
    Syntax {
        filetype: "markdown",
        filematch: &[".md", ".markdown"],
        keywords: &[],
        types: &[],
        constants: &[],
        singleline_comment_start: "",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[("```", "```"), ("~~~", "~~~")],
        quotes: "`",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[
            ("---", Highlight::Preprocessor),
            ("***", Highlight::Preprocessor),
            ("#", Highlight::Keyword1),
            (">", Highlight::Comment),
        ],
        aliases: &["md"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::STRINGS.union(HighlightFlags::FENCES),
    }
}
