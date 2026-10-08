use super::{HighlightFlags, Syntax};

pub fn css_syntax() -> Syntax {
    Syntax {
        filetype: "css",
        filematch: &[".css"],
        keywords: &[],
        types: &[
            "html", "body", "div", "span", "p", "a", "ul", "ol", "li", "table", "tr", "td", "th",
            "img", "h1", "h2", "h3", "h4", "h5", "h6", "header", "footer", "nav", "section",
            "article", "aside", "main", "form", "input", "button", "label", "select", "textarea",
            "pre", "code",
        ],
        constants: &[
            "inherit",
            "initial",
            "unset",
            "none",
            "auto",
            "transparent",
            "currentColor",
        ],
        singleline_comment_start: "",
        alt_comment_start: "",
        multiline_comment_start: "/*",
        multiline_comment_end: "*/",
        multiline_strings: &[],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &["@"],
        variable_prefixes: ".#",
        ident_chars: "-",
        operators: "",
        line_rules: &[],
        flags: HighlightFlags::NUMBERS
            .union(HighlightFlags::STRINGS)
            .union(HighlightFlags::FUNCTIONS)
            .union(HighlightFlags::CSS_VALUES),
    }
}
