use super::{HighlightFlags, Syntax};

pub fn shell_syntax() -> Syntax {
    Syntax {
        filetype: "shell",
        filematch: &[".sh", ".bash", ".zsh"],
        keywords: &[
            "if", "then", "else", "elif", "fi", "for", "while", "until", "do", "done", "case",
            "esac", "function", "return", "break", "continue", "export", "local", "readonly",
            "shift", "exit", "in", "select", "echo", "printf", "read", "source", "alias", "unset",
            "true|", "false|",
        ],
        singleline_comment_start: "#",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
