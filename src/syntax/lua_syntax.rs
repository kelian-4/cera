use super::{HighlightFlags, Syntax};

pub fn lua_syntax() -> Syntax {
    Syntax {
        filetype: "lua",
        filematch: &[".lua"],
        keywords: &[
            "function", "end", "if", "then", "else", "elseif", "for", "while", "repeat", "until",
            "do", "break", "return", "local", "in", "and", "or", "not", "require",
        ],
        types: &[],
        constants: &["true", "false", "nil"],
        singleline_comment_start: "--",
        alt_comment_start: "",
        multiline_comment_start: "--[[",
        multiline_comment_end: "]]",
        multiline_strings: &[("[[", "]]"), ("[=[", "]=]")],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "+-*/%^=<>~",
        line_rules: &[],
        aliases: &[],
        embed_tags: &[],
        string_embeds: &["html", "sql"],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS),
    }
}
