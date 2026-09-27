use super::{HighlightFlags, Syntax};

pub fn haskell_syntax() -> Syntax {
    Syntax {
        filetype: "haskell",
        filematch: &[".hs"],
        keywords: &[
            "module", "import", "where", "let", "in", "if", "then", "else", "case", "of", "data",
            "type", "newtype", "class", "instance", "deriving", "do", "infixl", "infixr", "infix",
            "Int|", "Integer|", "Double|", "Float|", "Bool|", "Char|", "String|", "Maybe|",
            "True|", "False|", "Nothing|", "Just|",
        ],
        singleline_comment_start: "--",
        multiline_comment_start: "{-",
        multiline_comment_end: "-}",
        flags: HighlightFlags::NUMBERS.union(HighlightFlags::STRINGS),
    }
}
