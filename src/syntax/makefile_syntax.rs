use super::{HighlightFlags, Syntax};

pub fn makefile_syntax() -> Syntax {
    Syntax {
        filetype: "makefile",
        filematch: &["Makefile", "makefile", "GNUmakefile", ".mk"],
        keywords: &[
            "ifeq", "ifneq", "ifdef", "ifndef", "else", "endif", "include", "sinclude", "define",
            "endef", "export", "unexport", "override", "private", "vpath",
        ],
        types: &[],
        constants: &[],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: ".",
        annotation_prefixes: &[],
        variable_prefixes: "$",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        aliases: &["make"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::KEY_VALUES),
    }
}
