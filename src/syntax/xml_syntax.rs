use super::{HighlightFlags, Syntax};

pub fn xml_syntax() -> Syntax {
    Syntax {
        filetype: "xml",
        filematch: &[
            ".xml", ".svg", ".xsd", ".xsl", ".xslt", ".plist", ".rss", ".atom", ".xaml", ".csproj",
            ".gpx", ".kml",
        ],
        keywords: &[],
        types: &[],
        constants: &[],
        singleline_comment_start: "",
        alt_comment_start: "",
        multiline_comment_start: "<!--",
        multiline_comment_end: "-->",
        multiline_strings: &[("<![CDATA[", "]]>")],
        quotes: "\"'",
        directive_prefix: "",
        annotation_prefixes: &[],
        variable_prefixes: "",
        ident_chars: "",
        operators: "",
        line_rules: &[],
        aliases: &["svg"],
        embed_tags: &[],
        string_embeds: &[],
        flags: HighlightFlags::STRINGS.union(HighlightFlags::MARKUP),
    }
}
