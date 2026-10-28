use crate::model::buffer::Buffer;

const SCAN_ROWS: usize = 1000;
const BLANK_LOOKAROUND: usize = 200;

fn default_unit(filetype: &str) -> usize {
    match filetype {
        "yaml" | "json" | "html" | "xml" | "css" | "scss" | "ruby" | "javascript"
        | "typescript" | "lisp" | "scheme" | "clojure" | "elixir" | "crystal" | "dart"
        | "coffeescript" | "graphql" | "nix" | "hcl" | "markdown" | "haskell" | "ocaml" => 2,
        _ => 4,
    }
}

fn leading_spaces(render: &str) -> Option<usize> {
    let mut n = 0usize;
    for c in render.chars() {
        if c == ' ' {
            n += 1;
        } else {
            return Some(n);
        }
    }
    None
}

/// Width (in render columns) of the leading whitespace of a row,
/// or `None` if the row is empty or only whitespace.
pub fn row_indent(buffer: &Buffer, idx: usize) -> Option<usize> {
    leading_spaces(buffer.row(idx).render())
}

/// Indent used to draw guides on a row. Blank rows take the smaller indent
/// of the nearest non-blank rows above and below, so guides stay continuous.
pub fn effective_indent(buffer: &Buffer, idx: usize) -> usize {
    if let Some(w) = row_indent(buffer, idx) {
        return w;
    }
    let mut above = 0usize;
    for i in (idx.saturating_sub(BLANK_LOOKAROUND)..idx).rev() {
        if let Some(w) = row_indent(buffer, i) {
            above = w;
            break;
        }
    }
    let mut below = 0usize;
    let end = (idx + 1 + BLANK_LOOKAROUND).min(buffer.num_rows());
    for i in idx + 1..end {
        if let Some(w) = row_indent(buffer, i) {
            below = w;
            break;
        }
    }
    above.min(below)
}

/// Detects the indentation unit (in render columns) used by the buffer:
/// one tab stop for tab-indented files, otherwise the most common indent
/// step between consecutive lines, falling back to the language's usual width.
pub fn detect_unit(buffer: &Buffer, filetype: &str, tab_stop: usize) -> usize {
    let mut tab_rows = 0usize;
    let mut space_rows = 0usize;
    let mut hist = [0usize; 9];
    let mut prev: Option<usize> = None;

    for row in buffer.rows().iter().take(SCAN_ROWS) {
        let chars = row.chars();
        let Some(w) = leading_spaces(row.render()) else {
            continue;
        };
        if chars.starts_with('\t') {
            tab_rows += 1;
            prev = Some(w);
            continue;
        }
        if w > 0 {
            space_rows += 1;
        }
        if let Some(p) = prev {
            if w > p {
                let delta = w - p;
                if (2..=8).contains(&delta) {
                    hist[delta] += 1;
                }
            }
        }
        prev = Some(w);
    }

    if tab_rows > space_rows {
        return tab_stop.max(1);
    }

    let mut best = 0usize;
    let mut best_count = 0usize;
    for (delta, &count) in hist.iter().enumerate().skip(2) {
        if count > best_count {
            best = delta;
            best_count = count;
        }
    }
    if best_count > 0 {
        best
    } else {
        default_unit(filetype)
    }
}

/// Whether a guide is drawn at render column `col` of a row indented by `indent`.
/// Only complete indentation levels get a guide.
pub fn is_guide_col(col: usize, indent: usize, unit: usize) -> bool {
    unit > 0 && col.is_multiple_of(unit) && col + unit <= indent
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buf(lines: &[&str]) -> Buffer {
        let mut b = Buffer::new();
        let v: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        b.set_lines(&v);
        b
    }

    #[test]
    fn detects_four_spaces() {
        let b = buf(&["fn a() {", "    if x {", "        y();", "    }", "}"]);
        assert_eq!(detect_unit(&b, "rust", 8), 4);
    }

    #[test]
    fn detects_two_spaces() {
        let b = buf(&["a:", "  b:", "    c: 1", "  d: 2"]);
        assert_eq!(detect_unit(&b, "yaml", 8), 2);
    }

    #[test]
    fn detects_tabs_as_tab_stop() {
        let b = buf(&["func a() {", "\tif x {", "\t\ty()", "\t}", "}"]);
        assert_eq!(detect_unit(&b, "go", 8), 8);
    }

    #[test]
    fn falls_back_to_language_default() {
        let b = buf(&["x", "y"]);
        assert_eq!(detect_unit(&b, "yaml", 8), 2);
        assert_eq!(detect_unit(&b, "rust", 8), 4);
    }

    #[test]
    fn ignores_single_space_comment_continuations() {
        let b = buf(&[
            "int f() {",
            "    /*",
            "     * text",
            "     */",
            "    return 0;",
            "}",
        ]);
        assert_eq!(detect_unit(&b, "c", 8), 4);
    }

    #[test]
    fn guide_columns_cover_complete_levels_only() {
        assert!(is_guide_col(0, 8, 4));
        assert!(is_guide_col(4, 8, 4));
        assert!(!is_guide_col(8, 8, 4));
        assert!(!is_guide_col(0, 2, 4));
        assert!(!is_guide_col(4, 6, 4));
    }

    #[test]
    fn blank_rows_use_smaller_neighbor_indent() {
        let b = buf(&["fn a() {", "        x();", "", "    y();", "}"]);
        assert_eq!(effective_indent(&b, 2), 4);
    }

    #[test]
    fn blank_rows_at_edges_have_no_guides() {
        let b = buf(&["", "    x();"]);
        assert_eq!(effective_indent(&b, 0), 0);
    }
}
