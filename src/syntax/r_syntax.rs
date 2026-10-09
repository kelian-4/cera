use super::{HighlightFlags, Syntax};

pub fn r_syntax() -> Syntax {
    Syntax {
        filetype: "r",
        filematch: &[".r", ".R"],
        keywords: &[
            "function", "if", "else", "for", "while", "repeat", "break", "next", "return",
            "library", "require", "in",
        ],
        types: &[],
        constants: &[
            "TRUE",
            "FALSE",
            "NULL",
            "NA",
            "NA_integer_",
            "NA_real_",
            "NA_character_",
            "Inf",
            "NaN",
            "T",
            "F",
        ],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: ".",
        operators: "+-*/^=<>!&|~?:%$@",
        line_rules: &[],
        aliases: &["rscript"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS),
    }
}
