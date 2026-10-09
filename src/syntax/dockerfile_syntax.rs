use super::{HighlightFlags, Syntax};

pub fn dockerfile_syntax() -> Syntax {
    Syntax {
        filetype: "dockerfile",
        filematch: &["Dockerfile", "Containerfile", ".dockerfile"],
        keywords: &[
            "from",
            "run",
            "cmd",
            "label",
            "maintainer",
            "expose",
            "env",
            "add",
            "copy",
            "entrypoint",
            "volume",
            "user",
            "workdir",
            "arg",
            "onbuild",
            "stopsignal",
            "healthcheck",
            "shell",
            "as",
        ],
        types: &[],
        constants: &[],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "$",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        aliases: &["docker"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::CASE_INSENSITIVE),
    }
}
