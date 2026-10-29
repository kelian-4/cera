use crate::model::buffer::Buffer;

const SCAN_ROWS: usize = 1000;
const LOOKAROUND: usize = 500;
const MAX_BLOCK_SCAN: usize = 2000;

pub fn default_unit(filetype: &str) -> usize {
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

fn starts_in_literal(buffer: &Buffer, idx: usize) -> bool {
    if idx == 0 {
        return false;
    }
    let s = buffer.row(idx - 1).hl_state;
    s.inner_comment || s.inner_string != 0 || s.comment || (s.string != 0 && s.embed == 0)
}

fn structural_indent(buffer: &Buffer, idx: usize) -> Option<usize> {
    if starts_in_literal(buffer, idx) {
        let mut j = idx - 1;
        let floor = idx.saturating_sub(LOOKAROUND);
        while j > floor && starts_in_literal(buffer, j) {
            j -= 1;
        }
        return Some(leading_spaces(buffer.row(j).render()).unwrap_or(0));
    }
    leading_spaces(buffer.row(idx).render())
}

pub fn row_has_indent(buffer: &Buffer, idx: usize) -> bool {
    structural_indent(buffer, idx).is_some()
}

pub fn effective_indent(buffer: &Buffer, idx: usize) -> usize {
    if let Some(w) = structural_indent(buffer, idx) {
        return w;
    }
    let start = idx.saturating_sub(LOOKAROUND);
    let has_prev = (start..idx).any(|i| structural_indent(buffer, i).is_some());
    if !has_prev {
        return 0;
    }
    let end = (idx + 1 + LOOKAROUND).min(buffer.num_rows());
    (idx + 1..end)
        .find_map(|i| structural_indent(buffer, i))
        .unwrap_or(0)
}

pub fn detect_unit(buffer: &Buffer, filetype: &str, tab_stop: usize) -> usize {
    let mut tab_rows = 0usize;
    let mut space_rows = 0usize;
    let mut hist = [0usize; 9];
    let mut prev: Option<usize> = None;

    for idx in 0..buffer.num_rows().min(SCAN_ROWS) {
        if starts_in_literal(buffer, idx) {
            continue;
        }
        let row = buffer.row(idx);
        let Some(w) = leading_spaces(row.render()) else {
            continue;
        };
        if row.chars().starts_with('\t') {
            tab_rows += 1;
            prev = Some(w);
            continue;
        }
        if w > 0 {
            space_rows += 1;
        }
        if let Some(p) = prev {
            if w > p && (2..=8).contains(&(w - p)) {
                hist[w - p] += 1;
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

pub fn is_guide_col(col: usize, indent: usize, unit: usize) -> bool {
    unit > 0 && col.is_multiple_of(unit) && col + unit <= indent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveGuide {
    pub col: usize,
    pub start: usize,
    pub end: usize,
}

impl ActiveGuide {
    pub fn covers(&self, col: usize, row: usize) -> bool {
        col == self.col && row >= self.start && row <= self.end
    }
}

fn next_structural_indent(buffer: &Buffer, idx: usize) -> Option<usize> {
    let end = (idx + 1 + LOOKAROUND).min(buffer.num_rows());
    (idx + 1..end).find_map(|i| structural_indent(buffer, i))
}

pub fn active_guide(buffer: &Buffer, cursor_row: usize, unit: usize) -> Option<ActiveGuide> {
    if unit == 0 || cursor_row >= buffer.num_rows() {
        return None;
    }
    let w = effective_indent(buffer, cursor_row);
    let is_code = row_has_indent(buffer, cursor_row);
    let header_col = w - w % unit;
    let is_header = is_code
        && next_structural_indent(buffer, cursor_row).is_some_and(|next| next >= header_col + unit);

    let col = if is_header {
        header_col
    } else if w >= unit {
        (w / unit - 1) * unit
    } else {
        return None;
    };
    let threshold = col + unit;

    let mut start = if is_header {
        cursor_row + 1
    } else {
        cursor_row
    };
    if !is_header {
        while start > 0
            && cursor_row - start < MAX_BLOCK_SCAN
            && effective_indent(buffer, start - 1) >= threshold
        {
            start -= 1;
        }
    }
    let mut end = cursor_row;
    while end + 1 < buffer.num_rows()
        && end - cursor_row < MAX_BLOCK_SCAN
        && effective_indent(buffer, end + 1) >= threshold
    {
        end += 1;
    }
    if start > end {
        return None;
    }
    Some(ActiveGuide { col, start, end })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::row::HlState;

    fn buf(lines: &[&str]) -> Buffer {
        let mut b = Buffer::new();
        let v: Vec<String> = lines.iter().map(|s| s.to_string()).collect();
        b.set_lines(&v);
        b
    }

    fn mark_comment(b: &mut Buffer, rows: &[usize]) {
        for &r in rows {
            b.row_mut(r).hl_state = HlState {
                comment: true,
                ..HlState::default()
            };
        }
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
    fn comment_text_does_not_skew_detection() {
        let mut b = buf(&[
            "int f() {",
            "    /*",
            "          deeply indented prose",
            "     */",
            "    return 0;",
            "}",
        ]);
        mark_comment(&mut b, &[1, 2]);
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
    fn blank_row_after_header_keeps_the_guide() {
        let b = buf(&["fn a() {", "", "    x();", "}"]);
        assert_eq!(effective_indent(&b, 1), 4);
    }

    #[test]
    fn blank_row_at_block_end_drops_to_next_indent() {
        let b = buf(&["fn a() {", "        x();", "", "    y();", "}"]);
        assert_eq!(effective_indent(&b, 2), 4);
    }

    #[test]
    fn blank_rows_at_edges_have_no_guides() {
        let b = buf(&["", "    x();", "", ""]);
        assert_eq!(effective_indent(&b, 0), 0);
        assert_eq!(effective_indent(&b, 2), 0);
        assert_eq!(effective_indent(&b, 3), 0);
    }

    #[test]
    fn comment_continuation_rows_inherit_opener_indent() {
        let mut b = buf(&["fn a() {", "    /*", " * text", "    */", "    x();", "}"]);
        mark_comment(&mut b, &[1, 2]);
        assert_eq!(effective_indent(&b, 2), 4);
        assert!(row_has_indent(&b, 2));
    }

    #[test]
    fn active_guide_on_header_is_the_block_it_opens() {
        let b = buf(&["fn a() {", "    x();", "    y();", "}", "z();"]);
        let g = active_guide(&b, 0, 4).unwrap();
        assert_eq!(
            g,
            ActiveGuide {
                col: 0,
                start: 1,
                end: 2
            }
        );
    }

    #[test]
    fn active_guide_in_body_is_the_innermost_block() {
        let b = buf(&[
            "fn a() {",
            "    if x {",
            "        y();",
            "        z();",
            "    }",
            "}",
        ]);
        let g = active_guide(&b, 3, 4).unwrap();
        assert_eq!(
            g,
            ActiveGuide {
                col: 4,
                start: 2,
                end: 3
            }
        );
        let g = active_guide(&b, 2, 4).unwrap();
        assert_eq!(g.col, 4);
    }

    #[test]
    fn active_guide_spans_blank_rows() {
        let b = buf(&["fn a() {", "    x();", "", "    y();", "}"]);
        let g = active_guide(&b, 1, 4).unwrap();
        assert_eq!(
            g,
            ActiveGuide {
                col: 0,
                start: 1,
                end: 3
            }
        );
    }

    #[test]
    fn no_active_guide_at_top_level() {
        let b = buf(&["a();", "b();"]);
        assert_eq!(active_guide(&b, 0, 4), None);
    }

    #[test]
    fn active_guide_covers_only_its_column_and_rows() {
        let g = ActiveGuide {
            col: 4,
            start: 2,
            end: 3,
        };
        assert!(g.covers(4, 2));
        assert!(!g.covers(0, 2));
        assert!(!g.covers(4, 4));
    }
}
