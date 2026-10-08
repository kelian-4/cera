use super::{HighlightFlags, Syntax};

pub fn elm_syntax() -> Syntax {
    Syntax {
        filetype: "elm",
        filematch: &[".elm"],
        keywords: &[
            "alias", "as", "case", "else", "exposing", "if", "import", "in", "let", "module", "of",
            "port", "then", "type", "where",
        ],
        types: &[
            "Int", "Float", "String", "Bool", "Char", "List", "Maybe", "Result", "Cmd", "Sub",
            "Html", "Msg", "Model", "Dict", "Set", "Array", "Order",
        ],
        constants: &["True", "False", "Nothing", "Just", "Ok", "Err"],
        singleline_comment_start: "--",
        alt_comment_start: "",
        multiline_comment_start: "{-",
        multiline_comment_end: "-}",
        multiline_strings: &[("\"\"\"", "\"\"\"")],
        quotes: "\"",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "'",
        operators: "+-*/=<>!&|^~?:",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::CAPITALIZED_TYPES)
            .union(HighlightFlags::CHAR_LITERALS),
    }
}
