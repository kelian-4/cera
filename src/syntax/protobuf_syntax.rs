use super::{HighlightFlags, Syntax};

pub fn protobuf_syntax() -> Syntax {
    Syntax {
        filetype: "protobuf",
        filematch: &[".proto"],
        keywords: &[
            "syntax",
            "package",
            "import",
            "option",
            "message",
            "enum",
            "service",
            "rpc",
            "returns",
            "stream",
            "extend",
            "extensions",
            "reserved",
            "oneof",
            "map",
            "repeated",
            "optional",
            "required",
            "default",
            "to",
            "max",
            "public",
            "weak",
        ],
        types: &[
            "double", "float", "int32", "int64", "uint32", "uint64", "sint32", "sint64", "fixed32",
            "fixed64", "sfixed32", "sfixed64", "bool", "string", "bytes",
        ],
        constants: &["true", "false"],
        singleline_comment_start: "//",
        alt_comment_start: "",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "+-*/%=<>!&|^~?:",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS)
            .union(HighlightFlags::CAPITALIZED_TYPES)
            .union(HighlightFlags::UPPER_CONSTANTS),
    }
}
