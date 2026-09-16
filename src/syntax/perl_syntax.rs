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
        singleline_comment_start: "#",
        multiline_comment_start: "",
        multiline_comment_end: "",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
