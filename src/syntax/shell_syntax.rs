use super::{HighlightFlags, Syntax};

pub fn shell_syntax() -> Syntax {
    Syntax {
        filetype: "shell",
        filematch: &[
            ".sh",
            ".bash",
            ".zsh",
            ".ksh",
            ".bashrc",
            ".zshrc",
            ".profile",
            ".bash_profile",
            ".bash_aliases",
            ".zprofile",
        ],
        keywords: &[
            "if", "then", "else", "elif", "fi", "for", "while", "until", "do", "done", "case",
            "esac", "function", "return", "break", "continue", "export", "local", "readonly",
            "shift", "exit", "in", "select", "echo|", "printf|", "read|", "source|", "alias|",
            "unset|", "cd|", "grep|", "sed|", "awk|", "cat|", "ls|",
        ],
        types: &[],
        constants: &["true", "false"],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[],
        quotes: "\"'`",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "$",
        ident_chars: "",
        operators: "|&<>;",
        line_rules: &[],
        aliases: &["bash", "sh", "zsh", "console", "shellscript"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS),
    }
}
