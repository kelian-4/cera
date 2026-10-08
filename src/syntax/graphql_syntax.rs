use super::{HighlightFlags, Syntax};

pub fn graphql_syntax() -> Syntax {
    Syntax {
        filetype: "graphql",
        filematch: &[".graphql", ".gql", ".graphqls"],
        keywords: &[
            "query",
            "mutation",
            "subscription",
            "fragment",
            "on",
            "schema",
            "extend",
            "directive",
            "scalar",
            "type",
            "interface",
            "union",
            "enum",
            "input",
            "implements",
            "repeatable",
        ],
        types: &["Int", "Float", "String", "Boolean", "ID"],
        constants: &["true", "false", "null"],
        singleline_comment_start: "#",
        alt_comment_start: "",
        multiline_comment_start: "",
        multiline_comment_end: "",
        multiline_strings: &[("\"\"\"", "\"\"\"")],
        quotes: "\"",
        directive_prefix: "",
        annotation_prefixes: &["@"],
        variable_prefixes: "$",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS)
            .union(HighlightFlags::CAPITALIZED_TYPES),
    }
}
