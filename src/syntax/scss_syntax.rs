use super::{HighlightFlags, Syntax};

pub fn scss_syntax() -> Syntax {
    Syntax {
        filetype: "scss",
        filematch: &[".scss", ".sass", ".less"],
        keywords: &["and", "or", "not", "from", "through", "to"],
        types: &[],
        constants: &[],
        singleline_comment_start: "//",
        alt_comment_start: "",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &["@"],
        variable_prefixes: "$",
        ident_chars: "-",
        operators: "",
        line_rules: &[],
        aliases: &["sass", "less"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS)
            .union(HighlightFlags::CSS_VALUES),
    }
}
