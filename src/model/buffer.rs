use super::row::Row;

pub struct CursorPos {
    pub row: usize,
    pub col: usize,
}

pub struct Buffer {
    rows: Vec<Row>,
    filename: Option<String>,
    dirty: usize,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            rows: vec![Row::new("")],
            filename: None,
            dirty: 0,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row(&self, i: usize) -> &Row {
        &self.rows[i]
    }

    pub fn row_mut(&mut self, i: usize) -> &mut Row {
        &mut self.rows[i]
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    fn reindex(&mut self, from: usize) {
        for (j, row) in self.rows.iter_mut().enumerate().skip(from) {
            row.idx = j;
        }
    }

    pub fn insert_row(&mut self, at: usize, s: &str) {
        if at > self.rows.len() {
            return;
        }
        self.rows.insert(at, Row::new(s));
        self.reindex(at);
        self.dirty += 1;
    }

    pub fn delete_row(&mut self, at: usize) {
        if at >= self.rows.len() {
            return;
        }
        self.rows.remove(at);
        self.reindex(at);
        self.dirty += 1;
    }

    pub fn insert_char(&mut self, row_idx: usize, col: usize, c: char) {
        if row_idx == self.rows.len() {
            self.insert_row(self.rows.len(), "");
        }
        self.rows[row_idx].insert_char(col, c);
        self.dirty += 1;
    }

    pub fn insert_newline(&mut self, row_idx: usize, col: usize) {
        if col == 0 {
            self.insert_row(row_idx, "");
        } else {
            let size = self.rows[row_idx].size();
            let tail = self.rows[row_idx].substr_by_char(col, size);
            let head = self.rows[row_idx].substr_by_char(0, col);
            self.insert_row(row_idx + 1, &tail);
            self.rows[row_idx].set_chars(head);
        }
        self.dirty += 1;
    }

    pub fn delete_char(&mut self, row_idx: usize, col: usize) -> CursorPos {
        if row_idx == self.rows.len() {
            return CursorPos { row: row_idx, col };
        }
        if col == 0 && row_idx == 0 {
            return CursorPos { row: row_idx, col };
        }

        if col > 0 {
            self.rows[row_idx].del_char(col - 1);
            self.dirty += 1;
            return CursorPos {
                row: row_idx,
                col: col - 1,
            };
        }

        let new_col = self.rows[row_idx - 1].size();
        let current_chars = self.rows[row_idx].chars().to_string();
        self.rows[row_idx - 1].append_string(&current_chars);
        self.delete_row(row_idx);
        self.dirty += 1;
        CursorPos {
            row: row_idx - 1,
            col: new_col,
        }
    }

    pub fn to_string_content(&self) -> String {
        let mut result = String::new();
        for r in &self.rows {
            result.push_str(r.chars());
            result.push('\n');
        }
        result
    }

    pub fn set_lines(&mut self, lines: &[String]) {
        if lines.is_empty() {
            self.rows = vec![Row::new("")];
        } else {
            self.rows = lines.iter().map(|l| Row::new(l.as_str())).collect();
        }
        self.reindex(0);
        self.dirty = 0;
    }

    pub fn dirty(&self) -> bool {
        self.dirty > 0
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = 0;
    }

    pub fn mark_dirty(&mut self) {
        self.dirty += 1;
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    pub fn set_filename(&mut self, name: impl Into<String>) {
        self.filename = Some(name.into());
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_newline_splits_row() {
        let mut buf = Buffer::new();
        buf.set_lines(&["helloworld".to_string()]);
        buf.insert_newline(0, 5);

        assert_eq!(buf.num_rows(), 2);
        assert_eq!(buf.row(0).chars(), "hello");
        assert_eq!(buf.row(1).chars(), "world");
    }

    #[test]
    fn test_delete_char_merges_rows() {
        let mut buf = Buffer::new();
        buf.set_lines(&["hello".to_string(), "world".to_string()]);

        let pos = buf.delete_char(1, 0);
        assert_eq!(buf.num_rows(), 1);
        assert_eq!(buf.row(0).chars(), "helloworld");
        assert_eq!(pos.row, 0);
        assert_eq!(pos.col, 5);
    }

    #[test]
    fn test_to_string_content() {
        let mut buf = Buffer::new();
        buf.set_lines(&["line1".to_string(), "line2".to_string()]);
        assert_eq!(buf.to_string_content(), "line1\nline2\n");
    }

    #[test]
    fn test_dirty_flag() {
        let mut buf = Buffer::new();
        assert!(!buf.dirty());
        buf.insert_row(0, "x");
        assert!(buf.dirty());
        buf.clear_dirty();
        assert!(!buf.dirty());
    }

    #[test]
    fn test_new_buffer_has_one_empty_line() {
        let buf = Buffer::new();
        assert_eq!(buf.num_rows(), 1);
        assert_eq!(buf.row(0).chars(), "");
    }

    #[test]
    fn test_set_lines_empty_still_has_one_line() {
        let mut buf = Buffer::new();
        buf.set_lines(&[]);
        assert_eq!(buf.num_rows(), 1);
        assert_eq!(buf.row(0).chars(), "");
    }
}
