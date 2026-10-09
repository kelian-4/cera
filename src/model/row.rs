use super::utf8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HlState {
    pub comment: bool,
    pub string: u8,
    pub embed: u8,
    pub embed_end: u8,
    pub inner_comment: bool,
    pub inner_string: u8,
    pub undecided: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Highlight {
    Normal,
    Comment,
    MlComment,
    Keyword1,
    Keyword2,
    String,
    Number,
    Match,
    Type,
    Function,
    Constant,
    Operator,
    Preprocessor,
    Variable,
    Escape,
}

pub struct Row {
    chars: String,
    render: String,
    render_byte_offsets: Vec<usize>,
    hl: Vec<Highlight>,
    pub hl_state: HlState,
    pub idx: usize,
}

use std::sync::atomic::{AtomicUsize, Ordering};

pub static TAB_STOP: AtomicUsize = AtomicUsize::new(8);

pub fn tab_stop() -> usize {
    TAB_STOP.load(Ordering::Relaxed)
}

pub fn set_tab_stop(value: usize) {
    TAB_STOP.store(if value > 0 { value } else { 8 }, Ordering::Relaxed);
}

impl Row {
    pub fn new(chars: impl Into<String>) -> Self {
        let chars = chars.into();
        let mut row = Row {
            chars,
            render: String::new(),
            render_byte_offsets: Vec::new(),
            hl: Vec::new(),
            hl_state: HlState::default(),
            idx: 0,
        };
        row.update_render();
        row
    }

    pub fn chars(&self) -> &str {
        &self.chars
    }

    pub fn set_chars(&mut self, chars: impl Into<String>) {
        self.chars = chars.into();
        self.update_render();
    }

    pub fn render(&self) -> &str {
        &self.render
    }

    pub fn hl(&self) -> &[Highlight] {
        &self.hl
    }

    pub fn hl_mut(&mut self) -> &mut Vec<Highlight> {
        &mut self.hl
    }

    pub fn size(&self) -> usize {
        utf8::codepoint_count(&self.chars)
    }

    pub fn rsize(&self) -> usize {
        utf8::codepoint_count(&self.render)
    }

    pub fn render_byte_offsets(&self) -> &[usize] {
        &self.render_byte_offsets
    }

    pub fn render_char_width(&self, render_char_idx: usize) -> usize {
        match self.render.chars().nth(render_char_idx) {
            Some(ch) => {
                if ch == ' ' {
                    1
                } else {
                    utf8::display_width(ch).max(1)
                }
            }
            None => 0,
        }
    }

    pub fn insert_char(&mut self, at: usize, c: char) {
        let at = at.min(self.size());
        let byte_off = utf8::byte_offset_for_char_index(&self.chars, at);
        self.chars.insert(byte_off, c);
        self.update_render();
    }

    pub fn del_char(&mut self, at: usize) {
        if at >= self.size() {
            return;
        }
        let byte_off = utf8::byte_offset_for_char_index(&self.chars, at);
        self.chars.remove(byte_off);
        self.update_render();
    }

    pub fn append_string(&mut self, s: &str) {
        self.chars.push_str(s);
        self.update_render();
    }

    pub fn substr_by_char(&self, from_char_idx: usize, to_char_idx: usize) -> String {
        utf8::substr_by_char(&self.chars, from_char_idx, to_char_idx)
    }

    pub fn cx_to_rx(&self, cx: usize) -> usize {
        let mut rx = 0usize;
        let limit = cx.min(self.size());
        for c in self.chars.chars().take(limit) {
            if c == '\t' {
                rx += (tab_stop() - 1) - (rx % tab_stop());
                rx += 1;
            } else {
                rx += utf8::display_width(c);
            }
        }
        rx
    }

    pub fn rx_to_cx(&self, rx: usize) -> usize {
        let mut cur_rx = 0usize;
        let mut cx = 0usize;
        for c in self.chars.chars() {
            if cur_rx > rx {
                break;
            }
            if c == '\t' {
                cur_rx += (tab_stop() - 1) - (cur_rx % tab_stop());
                cur_rx += 1;
            } else {
                cur_rx += utf8::display_width(c);
            }
            if cur_rx > rx {
                break;
            }
            cx += 1;
        }
        cx
    }

    pub fn update_render(&mut self) {
        let mut render = String::with_capacity(self.chars.len());

        for c in self.chars.chars() {
            if c == '\t' {
                let col = utf8::codepoint_count(&render);
                render.push(' ');
                let mut new_col = col + 1;
                while !new_col.is_multiple_of(tab_stop()) {
                    render.push(' ');
                    new_col += 1;
                }
            } else {
                render.push(c);
            }
        }

        self.render_byte_offsets = render.char_indices().map(|(i, _)| i).collect();
        self.render_byte_offsets.push(render.len());

        let render_char_count = utf8::codepoint_count(&render);
        self.render = render;
        self.hl = vec![Highlight::Normal; render_char_count];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_delete() {
        let mut row = Row::new("helo");
        row.insert_char(3, 'l');
        assert_eq!(row.chars(), "hello");
        row.del_char(0);
        assert_eq!(row.chars(), "ello");
    }

    #[test]
    fn test_tab_expansion() {
        set_tab_stop(8);
        let row = Row::new("a\tb");
        assert_eq!(utf8::codepoint_count(row.render()), 9);
        assert_eq!(row.render().chars().next(), Some('a'));
        assert_eq!(row.render().chars().nth(8), Some('b'));
    }

    #[test]
    fn test_cx_rx_roundtrip() {
        set_tab_stop(8);
        let row = Row::new("a\tbc");
        let rx = row.cx_to_rx(3);
        let cx = row.rx_to_cx(rx);
        assert_eq!(cx, 3);
    }

    #[test]
    fn test_utf8_insert_delete() {
        let mut row = Row::new("a");
        row.insert_char(1, '\u{e9}');
        assert_eq!(row.size(), 2);
        assert_eq!(row.chars(), "a\u{e9}");
        row.del_char(1);
        assert_eq!(row.chars(), "a");
        assert_eq!(row.size(), 1);
    }
}
