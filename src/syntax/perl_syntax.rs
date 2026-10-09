use super::{HighlightFlags, Syntax};

pub fn perl_syntax() -> Syntax {
    Syntax {
        filetype: "perl",
        filematch: &[".pl", ".pm"],
        keywords: &[
            "sub", "my", "our", "local", "if", "elsif", "else", "unless", "while", "until", "for",
            "foreach", "do", "last", "next", "redo", "return", "package", "use", "require", "qw",
            "shift", "push", "pop", "die", "undef|",
        ],
        types: &[],
        constants: &[],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "=pod",
        multiline_comment_end: "=cut",
        multiline_strings: &[],
        quotes: "\"'`",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "$@%",
        ident_chars: "",
        operators: "+-*/%=<>!&|^~?:",
        line_rules: &[],
        aliases: &[],
        embed_tags: &[],
        string_embeds: &["html", "sql"],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS),
    }
}
