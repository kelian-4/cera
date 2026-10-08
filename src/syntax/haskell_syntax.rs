use super::{HighlightFlags, Syntax};

pub fn haskell_syntax() -> Syntax {
    Syntax {
        filetype: "haskell",
        filematch: &[".hs"],
        keywords: &[
            "module", "import", "where", "let", "in", "if", "then", "else", "case", "of", "data",
            "type", "newtype", "class", "instance", "deriving", "do", "infixl", "infixr", "infix",
        ],
        types: &[
            "Int", "Integer", "Float", "Double", "Bool", "Char", "String", "Maybe", "Either", "IO",
            "Ordering", "Word",
        ],
        constants: &[
            "True", "False", "Nothing", "Just", "Left", "Right", "LT", "GT", "EQ",
        ],
        singleline_comment_start: "--",
        alt_comment_start: "",
        multiline_comment_start: "{-",
        multiline_comment_end: "-}",
        multiline_strings: &[],
        quotes: "\"",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "'",
        operators: "+-*/=<>!&|^~$",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::CAPITALIZED_TYPES)
            .union(HighlightFlags::CHAR_LITERALS),
    }
}
