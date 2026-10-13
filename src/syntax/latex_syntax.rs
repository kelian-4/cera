use super::{HighlightFlags, Syntax};

pub fn latex_syntax() -> Syntax {
    Syntax {
        filetype: "latex",
        filematch: &[".tex", ".sty", ".ltx"],
        keywords: &[],
        types: &[],
        constants: &[],
        singleline_comment_start: "%",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[("\\begin{verbatim}", "\\end{verbatim}")],
        quotes: "$",
        directive_prefix: "",
        annotation_prefixes: &["\\"],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        aliases: &["tex"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::STRINGS,
    }
}
