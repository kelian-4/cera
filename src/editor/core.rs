use std::io;

use crate::editor::buffer_manager::BufferManager;
use crate::editor::clipboard::Clipboard;
use crate::editor::undo_stack::{EditAction, EditKind};
use crate::io::config::Config;
use crate::io::file_io;
use crate::model::row::{self};
use crate::model::selection::Position;
use crate::model::utf8;
use crate::syntax;
use crate::terminal::{self, MouseEvent, Terminal};
use crate::view::help_text::MessageKind;
use crate::view::screen_renderer::{self, SearchDisplayInfo, ViewState};

fn ctrl(c: char) -> i32 {
    (c as i32) & 0x1f
}

pub struct Editor {
    terminal: Terminal,
    buffer_manager: BufferManager,
    syntax_db: Vec<syntax::Syntax>,
    clipboard: Clipboard,
    config: Config,

    screenrows: usize,
    screencols: usize,

    status_message: String,
    status_message_kind: MessageKind,
    status_message_time: std::time::Instant,

    quit_times: i32,

    overlay_full_doc: bool,
    help_active: bool,
    prompt_text: Option<String>,

    search_active: bool,
    search_term: String,
    search_current_match: usize,
    search_total_matches: usize,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        let (config, _loaded) =
            crate::io::config::load_from_file(&crate::io::config::default_config_path());
        row::set_tab_stop(config.tab_stop);

        let mut terminal = Terminal::new()?;
        let (rows, cols) = terminal.get_window_size()?;

        if config.mouse_enabled {
            terminal.enable_mouse_reporting();
        }
        Terminal::install_resize_handler();

        let syntax_db = syntax::build_database();
        let buffer_manager = BufferManager::new();

        Ok(Editor {
            terminal,
            buffer_manager,
            syntax_db,
            clipboard: Clipboard::new(),
            quit_times: config.quit_times,
            config,
            screenrows: rows as usize,
            screencols: cols as usize,
            status_message: String::new(),
            status_message_kind: MessageKind::Info,
            status_message_time: std::time::Instant::now(),
            overlay_full_doc: false,
            help_active: false,
            prompt_text: None,
            search_active: false,
            search_term: String::new(),
            search_current_match: 0,
            search_total_matches: 0,
        })
    }

    pub fn open_file(&mut self, filename: &str) {
        self.buffer_manager.open_file(filename, &self.syntax_db);
    }

    fn set_status_message(&mut self, msg: impl Into<String>) {
        self.set_status_message_kind(msg, MessageKind::Info);
    }

    fn set_status_message_kind(&mut self, msg: impl Into<String>, kind: MessageKind) {
        self.status_message = msg.into();
        self.status_message_kind = kind;
        self.status_message_time = std::time::Instant::now();
    }

    fn apply_resize_if_needed(&mut self) {
        if Terminal::consume_resize_flag() {
            if let Ok((rows, cols)) = self.terminal.get_window_size() {
                self.screenrows = rows as usize;
                self.screencols = cols as usize;
            }
        }
    }

    fn build_view_state(&self, search: SearchDisplayInfo) -> ViewState<'_> {
        let ob = self.buffer_manager.active();
        ViewState {
            buffer_manager: &self.buffer_manager,
            rowoff: ob.rowoff,
            coloff: ob.coloff,
            screenrows: self.screenrows,
            screencols: self.screencols,
            status_message: self.status_message.clone(),
            status_message_kind: self.status_message_kind,
            status_message_age_seconds: self.status_message_time.elapsed().as_secs() as i64,
            overlay_full_doc: self.overlay_full_doc,
            help_active: self.help_active,
            prompt_text: self.prompt_text.clone(),
            search,
            search_term: self.search_term.clone(),
            config: &self.config,
        }
    }

    fn scroll(&mut self) {
        let search = SearchDisplayInfo::default();
        let state = self.build_view_state(search);
        let text_rows = screen_renderer::text_area_screen_rows(&state);
        let gutter = screen_renderer::gutter_width(&state);
        let text_cols = self.screencols.saturating_sub(gutter);

        let ob = self.buffer_manager.active_mut();

        ob.cursor.rx = if ob.cursor.cy < ob.buffer.num_rows() {
            ob.buffer.row(ob.cursor.cy).cx_to_rx(ob.cursor.cx)
        } else {
            0
        };

        if ob.cursor.cy < ob.rowoff {
            ob.rowoff = ob.cursor.cy;
        }
        if ob.cursor.cy >= ob.rowoff + text_rows {
            ob.rowoff = ob.cursor.cy + 1 - text_rows;
        }
        if ob.cursor.rx < ob.coloff {
            ob.coloff = ob.cursor.rx;
        }
        if ob.cursor.rx >= ob.coloff + text_cols {
            ob.coloff = ob.cursor.rx + 1 - text_cols;
        }
    }

    fn refresh_screen(&mut self) {
        self.apply_resize_if_needed();
        self.scroll();

        let search = SearchDisplayInfo {
            active: self.search_active,
            current_index: self.search_current_match,
            total_matches: self.search_total_matches,
        };
        let state = self.build_view_state(search);
        let output = screen_renderer::refresh(&state);
        self.terminal.write(output.as_bytes());
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.refresh_screen();
            self.process_keypress();
        }
    }

    fn move_cursor(&mut self, key: i32, extend_selection: bool) {
        let ob = self.buffer_manager.active_mut();
        let cursor = &mut ob.cursor;

        if extend_selection && !ob.selection.is_active {
            ob.selection.is_active = true;
            ob.selection.anchor = Position::new(cursor.cy, cursor.cx);
        } else if !extend_selection {
            ob.selection.is_active = false;
        }

        let mut has_row = cursor.cy < ob.buffer.num_rows();

        match key {
            terminal::ARROW_LEFT => {
                if cursor.cx != 0 {
                    cursor.cx -= 1;
                } else if cursor.cy > 0 {
                    cursor.cy -= 1;
                    cursor.cx = ob.buffer.row(cursor.cy).size();
                }
            }
            terminal::ARROW_RIGHT => {
                if has_row && cursor.cx < ob.buffer.row(cursor.cy).size() {
                    cursor.cx += 1;
                } else if has_row
                    && cursor.cx == ob.buffer.row(cursor.cy).size()
                    && cursor.cy + 1 < ob.buffer.num_rows()
                {
                    cursor.cy += 1;
                    cursor.cx = 0;
                }
            }
            terminal::ARROW_UP => {
                if cursor.cy != 0 {
                    cursor.cy -= 1;
                }
            }
            terminal::ARROW_DOWN if cursor.cy + 1 < ob.buffer.num_rows() => {
                cursor.cy += 1;
            }
            _ => {}
        }

        has_row = cursor.cy < ob.buffer.num_rows();
        let rowlen = if has_row {
            ob.buffer.row(cursor.cy).size()
        } else {
            0
        };
        if cursor.cx > rowlen {
            cursor.cx = rowlen;
        }

        if extend_selection {
            ob.selection.active = Position::new(cursor.cy, cursor.cx);
        }

        ob.undo_stack.break_coalescing();
    }

    fn text_in_range(&self, from: Position, to: Position) -> String {
        let buf = &self.buffer_manager.active().buffer;
        if from.row == to.row {
            return buf.row(from.row).substr_by_char(from.col, to.col);
        }
        let mut result = buf
            .row(from.row)
            .substr_by_char(from.col, buf.row(from.row).size());
        result.push('\n');
        for r in (from.row + 1)..to.row {
            result.push_str(buf.row(r).chars());
            result.push('\n');
        }
        result.push_str(&buf.row(to.row).substr_by_char(0, to.col));
        result
    }

    fn insert_text_at(&mut self, pos: Position, text: &str) -> Position {
        let mut row = pos.row;
        let mut col = pos.col;

        let segments: Vec<&str> = text.split('\n').collect();
        let last_idx = segments.len() - 1;

        for (seg_idx, segment) in segments.iter().enumerate() {
            for c in segment.chars() {
                self.buffer_manager
                    .active_mut()
                    .buffer
                    .insert_char(row, col, c);
                col += 1;
            }
            if seg_idx != last_idx {
                self.buffer_manager
                    .active_mut()
                    .buffer
                    .insert_newline(row, col);
                row += 1;
                col = 0;
            }
        }

        let active_syntax_idx = self.buffer_manager.active().active_syntax_idx;
        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            let buf = &mut self.buffer_manager.active_mut().buffer;
            let max_row = row.min(buf.num_rows().saturating_sub(1));
            for r in pos.row..=max_row {
                syntax::update_syntax(buf, r, Some(syn));
            }
        }

        Position::new(row, col)
    }

    fn delete_selection_for_undo(&mut self, from: Position, to: Position) {
        let active_syntax_idx = self.buffer_manager.active().active_syntax_idx;

        {
            let ob = self.buffer_manager.active_mut();
            let buf = &mut ob.buffer;

            if from.row == to.row {
                for i in (from.col..to.col).rev() {
                    buf.delete_char(from.row, i + 1);
                }
            } else {
                let tail_after = buf
                    .row(to.row)
                    .substr_by_char(to.col, buf.row(to.row).size());
                let head = buf.row(from.row).substr_by_char(0, from.col);
                for r in (from.row + 1..=to.row).rev() {
                    buf.delete_row(r);
                }
                let combined = format!("{}{}", head, tail_after);
                buf.row_mut(from.row).set_chars(combined);
            }
        }

        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            let buf = &mut self.buffer_manager.active_mut().buffer;
            if from.row < buf.num_rows() {
                syntax::update_syntax(buf, from.row, Some(syn));
            }
        }
    }

    fn delete_selection(&mut self) {
        let ob = self.buffer_manager.active();
        if !ob.selection.is_active || ob.selection.is_empty() {
            return;
        }

        let from = ob.selection.start();
        let to = ob.selection.end();
        let removed = self.text_in_range(from, to);

        self.delete_selection_for_undo(from, to);

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = from.row;
        ob.cursor.cx = from.col;
        ob.selection.is_active = false;

        let action = EditAction {
            start: from,
            removed_text: removed,
            cursor_before: to,
            cursor_after: from,
            kind: EditKind::DeleteBlock,
            ..Default::default()
        };
        ob.undo_stack.push(action);
    }

    fn insert_codepoint(&mut self, c: char) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if has_selection {
            self.delete_selection();
        }

        let ob = self.buffer_manager.active();
        let before = Position::new(ob.cursor.cy, ob.cursor.cx);

        if ob.cursor.cy == ob.buffer.num_rows() {
            let n = ob.buffer.num_rows();
            self.buffer_manager.active_mut().buffer.insert_row(n, "");
        }

        let ob = self.buffer_manager.active_mut();
        ob.buffer.insert_char(ob.cursor.cy, ob.cursor.cx, c);

        let active_syntax_idx = ob.active_syntax_idx;
        let cursor_row = ob.cursor.cy;
        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            syntax::update_syntax(
                &mut self.buffer_manager.active_mut().buffer,
                cursor_row,
                Some(syn),
            );
        }

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cx += 1;

        let action = EditAction {
            start: before,
            inserted_text: c.to_string(),
            cursor_before: before,
            cursor_after: Position::new(ob.cursor.cy, ob.cursor.cx),
            kind: EditKind::InsertChar,
            ..Default::default()
        };
        ob.undo_stack.push(action);
    }

    fn insert_newline(&mut self) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if has_selection {
            self.delete_selection();
        }

        let ob = self.buffer_manager.active();
        let before = Position::new(ob.cursor.cy, ob.cursor.cx);
        let (cy, cx) = (ob.cursor.cy, ob.cursor.cx);

        self.buffer_manager
            .active_mut()
            .buffer
            .insert_newline(cy, cx);

        let ob = self.buffer_manager.active();
        let active_syntax_idx = ob.active_syntax_idx;
        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            let buf = &mut self.buffer_manager.active_mut().buffer;
            syntax::update_syntax(buf, cy, Some(syn));
            if cy + 1 < buf.num_rows() {
                syntax::update_syntax(buf, cy + 1, Some(syn));
            }
        }

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy += 1;
        ob.cursor.cx = 0;

        let action = EditAction {
            start: before,
            inserted_text: "\n".to_string(),
            cursor_before: before,
            cursor_after: Position::new(ob.cursor.cy, ob.cursor.cx),
            kind: EditKind::InsertNewline,
            ..Default::default()
        };
        ob.undo_stack.push(action);
    }

    fn delete_char_backward(&mut self) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if has_selection {
            self.delete_selection();
            return;
        }

        let ob = self.buffer_manager.active();
        if ob.cursor.cy == ob.buffer.num_rows() {
            return;
        }
        if ob.cursor.cx == 0 && ob.cursor.cy == 0 {
            return;
        }

        let before = Position::new(ob.cursor.cy, ob.cursor.cx);
        let removed = if ob.cursor.cx > 0 {
            ob.buffer
                .row(ob.cursor.cy)
                .substr_by_char(ob.cursor.cx - 1, ob.cursor.cx)
        } else {
            "\n".to_string()
        };

        let (cy, cx) = (ob.cursor.cy, ob.cursor.cx);
        let pos = self.buffer_manager.active_mut().buffer.delete_char(cy, cx);

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = pos.row;
        ob.cursor.cx = pos.col;

        let active_syntax_idx = ob.active_syntax_idx;
        let cursor_row = ob.cursor.cy;
        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            let buf = &mut self.buffer_manager.active_mut().buffer;
            if cursor_row < buf.num_rows() {
                syntax::update_syntax(buf, cursor_row, Some(syn));
            }
        }

        let ob = self.buffer_manager.active_mut();
        let action = EditAction {
            start: Position::new(ob.cursor.cy, ob.cursor.cx),
            removed_text: removed,
            cursor_before: before,
            cursor_after: Position::new(ob.cursor.cy, ob.cursor.cx),
            kind: EditKind::DeleteCharBackward,
            ..Default::default()
        };
        ob.undo_stack.push(action);
    }

    fn delete_char_forward(&mut self) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if has_selection {
            self.delete_selection();
            return;
        }

        let ob = self.buffer_manager.active();
        if ob.cursor.cy >= ob.buffer.num_rows() {
            return;
        }
        let at_line_end = ob.cursor.cx == ob.buffer.row(ob.cursor.cy).size();
        if at_line_end && ob.cursor.cy == ob.buffer.num_rows() - 1 {
            return;
        }

        let before = Position::new(ob.cursor.cy, ob.cursor.cx);

        let (target_row, target_col, removed) = if !at_line_end {
            let removed = ob
                .buffer
                .row(ob.cursor.cy)
                .substr_by_char(ob.cursor.cx, ob.cursor.cx + 1);
            (ob.cursor.cy, ob.cursor.cx + 1, removed)
        } else {
            (ob.cursor.cy + 1, 0, "\n".to_string())
        };

        let pos = self
            .buffer_manager
            .active_mut()
            .buffer
            .delete_char(target_row, target_col);

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = pos.row;
        ob.cursor.cx = pos.col;

        let active_syntax_idx = ob.active_syntax_idx;
        let cursor_row = ob.cursor.cy;
        if let Some(sidx) = active_syntax_idx {
            let syn = &self.syntax_db[sidx];
            let buf = &mut self.buffer_manager.active_mut().buffer;
            if cursor_row < buf.num_rows() {
                syntax::update_syntax(buf, cursor_row, Some(syn));
            }
        }

        let ob = self.buffer_manager.active_mut();
        let action = EditAction {
            start: before,
            removed_text: removed,
            cursor_before: before,
            cursor_after: Position::new(ob.cursor.cy, ob.cursor.cx),
            kind: EditKind::DeleteCharForward,
            ..Default::default()
        };
        ob.undo_stack.push(action);
    }

    fn cut_current_line(&mut self) {
        let ob = self.buffer_manager.active();
        if ob.buffer.num_rows() == 0 {
            return;
        }

        let row = ob.cursor.cy;
        if row >= ob.buffer.num_rows() {
            return;
        }

        let last_row = row == ob.buffer.num_rows() - 1;
        let line_text = ob.buffer.row(row).chars().to_string();

        let (from, to, removed) = if last_row && ob.buffer.num_rows() == 1 {
            (
                Position::new(row, 0),
                Position::new(row, ob.buffer.row(row).size()),
                line_text.clone(),
            )
        } else if last_row {
            let prev_size = ob.buffer.row(row - 1).size();
            (
                Position::new(row - 1, prev_size),
                Position::new(row, ob.buffer.row(row).size()),
                format!("\n{}", line_text),
            )
        } else {
            (
                Position::new(row, 0),
                Position::new(row + 1, 0),
                format!("{}\n", line_text),
            )
        };

        self.clipboard.copy(&line_text);
        self.delete_selection_for_undo(from, to);

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = from.row;
        ob.cursor.cx = from.col;
        ob.selection.is_active = false;

        let action = EditAction {
            start: from,
            removed_text: removed,
            cursor_before: Position::new(row, ob.cursor.cx),
            cursor_after: from,
            kind: EditKind::DeleteBlock,
            ..Default::default()
        };
        ob.undo_stack.push(action);

        self.set_status_message("Ligne coupee");
    }

    fn save(&mut self) {
        let has_filename = self.buffer_manager.active().buffer.filename().is_some();

        if !has_filename {
            let filename = self.prompt("Sauvegarder sous : %s (Echap pour annuler)");
            let filename = match filename {
                Some(f) if !f.is_empty() => f,
                _ => {
                    self.set_status_message("Sauvegarde annulee");
                    return;
                }
            };

            self.buffer_manager
                .active_mut()
                .buffer
                .set_filename(&filename);
            let syntax_idx = self
                .syntax_db
                .iter()
                .position(|s| syntax::matches(s, &filename));
            self.buffer_manager.active_mut().active_syntax_idx = syntax_idx;

            if let Some(sidx) = syntax_idx {
                let syn = &self.syntax_db[sidx];
                let buf = &mut self.buffer_manager.active_mut().buffer;
                let n = buf.num_rows();
                for r in 0..n {
                    syntax::update_syntax(buf, r, Some(syn));
                }
            }
        }

        let filename = self
            .buffer_manager
            .active()
            .buffer
            .filename()
            .unwrap()
            .to_string();
        match file_io::save_file(&mut self.buffer_manager.active_mut().buffer) {
            Ok(()) => {
                self.set_status_message_kind(filename, MessageKind::Success);
            }
            Err(e) => {
                self.set_status_message_kind(
                    format!("Erreur de sauvegarde : {}", e),
                    MessageKind::Error,
                );
            }
        }
    }

    fn prompt(&mut self, prompt_fmt: &str) -> Option<String> {
        let mut buf = String::new();

        loop {
            let msg = prompt_fmt.replace("%s", &buf);
            self.prompt_text = Some(msg);
            self.refresh_screen();

            let c = match self.terminal.read_key() {
                Ok(k) => k,
                Err(_) => terminal::NO_KEY,
            };

            if c == terminal::NO_KEY {
                self.apply_resize_if_needed();
                continue;
            }

            if c == terminal::DEL_KEY || c == ctrl('h') || c == terminal::BACKSPACE {
                buf.pop();
            } else if c == 0x1b {
                self.prompt_text = None;
                return None;
            } else if c == '\r' as i32 {
                if !buf.is_empty() {
                    self.prompt_text = None;
                    return Some(buf);
                }
            } else if terminal::is_encoded_codepoint(c) {
                buf.push(terminal::decode_codepoint(c));
            } else if !(c as u8 as char).is_control() && c < 128 {
                buf.push(c as u8 as char);
            }
        }
    }

    fn clear_find_highlight(&mut self) {
        let active_syntax_idx = self.buffer_manager.active().active_syntax_idx;
        let num_rows = self.buffer_manager.active().buffer.num_rows();

        for r in 0..num_rows {
            if let Some(sidx) = active_syntax_idx {
                let syn = &self.syntax_db[sidx];
                syntax::update_syntax(&mut self.buffer_manager.active_mut().buffer, r, Some(syn));
            } else {
                let buf = &mut self.buffer_manager.active_mut().buffer;
                let hl = buf.row_mut(r).hl_mut();
                hl.iter_mut()
                    .for_each(|h| *h = crate::model::row::Highlight::Normal);
            }
        }
    }

    fn update_find_highlight(&mut self) {
        self.search_total_matches = 0;
        if self.search_term.is_empty() {
            return;
        }

        let ob = self.buffer_manager.active();
        let cursor_row = ob.cursor.cy;
        let cursor_col = ob.cursor.cx;
        let num_rows = ob.buffer.num_rows();
        let term = self.search_term.clone();

        let mut match_containing_cursor = 0usize;

        for r in 0..num_rows {
            let render = self
                .buffer_manager
                .active()
                .buffer
                .row(r)
                .render()
                .to_string();
            let mut search_from = 0usize;
            while let Some(abs_pos) = utf8::find_pattern(&render, &term, search_from) {
                self.search_total_matches += 1;

                match r.cmp(&cursor_row) {
                    std::cmp::Ordering::Equal => {
                        let match_char_idx = utf8::char_index_for_byte_offset(&render, abs_pos);
                        if match_char_idx <= cursor_col {
                            match_containing_cursor = self.search_total_matches;
                        }
                    }
                    std::cmp::Ordering::Less => {
                        match_containing_cursor = self.search_total_matches;
                    }
                    std::cmp::Ordering::Greater => {}
                }

                search_from = abs_pos + 1;
            }
        }

        self.search_current_match = match_containing_cursor;
    }

    fn find_next(&mut self, direction: i32) {
        if self.search_term.is_empty() {
            return;
        }

        let ob = self.buffer_manager.active();
        let start_row = ob.cursor.cy;
        let start_col = ob.cursor.cx;
        let num_rows = ob.buffer.num_rows();
        if num_rows == 0 {
            return;
        }
        let term = self.search_term.clone();

        let mut row = start_row;
        for step in 0..=num_rows {
            let is_first_pass = step == 0;
            let is_final_wrap = step == num_rows;
            let render = self
                .buffer_manager
                .active()
                .buffer
                .row(row)
                .render()
                .to_string();

            if direction > 0 {
                let from_col = if is_first_pass { start_col + 1 } else { 0 };
                let byte_from = utf8::byte_offset_for_char_index(&render, from_col);
                if byte_from <= render.len() {
                    if let Some(abs_pos) = utf8::find_pattern(&render, &term, byte_from) {
                        let match_col = utf8::char_index_for_byte_offset(&render, abs_pos);
                        if !(is_final_wrap && match_col > start_col) {
                            let ob = self.buffer_manager.active_mut();
                            ob.cursor.cy = row;
                            ob.cursor.cx = match_col;
                            self.update_find_highlight();
                            return;
                        }
                    }
                }
            } else {
                let upto = if is_first_pass {
                    start_col
                } else {
                    self.buffer_manager.active().buffer.row(row).size() + 1
                };
                let byte_upto = utf8::byte_offset_for_char_index(&render, upto).min(render.len());
                if let Some(pos) = utf8::rfind_pattern_before(&render, &term, byte_upto) {
                    let match_col = utf8::char_index_for_byte_offset(&render, pos);
                    if !(is_final_wrap && match_col < start_col) {
                        let ob = self.buffer_manager.active_mut();
                        ob.cursor.cy = row;
                        ob.cursor.cx = match_col;
                        self.update_find_highlight();
                        return;
                    }
                }
            }

            row = if direction > 0 {
                (row + 1) % num_rows
            } else {
                (row + num_rows - 1) % num_rows
            };
        }

        self.set_status_message("Aucune occurrence trouvee");
    }

    fn find(&mut self) {
        let start_pos = {
            let ob = self.buffer_manager.active();
            Position::new(ob.cursor.cy, ob.cursor.cx)
        };
        self.search_active = true;
        self.search_term.clear();
        self.set_status_message("");

        loop {
            self.refresh_screen();

            let c = match self.terminal.read_key() {
                Ok(k) => k,
                Err(_) => terminal::NO_KEY,
            };

            if c == terminal::NO_KEY {
                self.apply_resize_if_needed();
                continue;
            }

            if c == 0x1b {
                break;
            }
            if c == '\r' as i32 {
                if !self.search_term.is_empty() {
                    self.find_next(1);
                }
                continue;
            }
            if c == terminal::ARROW_DOWN || c == terminal::ARROW_RIGHT {
                if !self.search_term.is_empty() {
                    self.find_next(1);
                }
                continue;
            }
            if c == terminal::ARROW_UP || c == terminal::ARROW_LEFT {
                if !self.search_term.is_empty() {
                    self.find_next(-1);
                }
                continue;
            }

            if c == terminal::DEL_KEY || c == ctrl('h') || c == terminal::BACKSPACE {
                self.search_term.pop();
            } else if terminal::is_encoded_codepoint(c) {
                self.search_term.push(terminal::decode_codepoint(c));
            } else if !(c as u8 as char).is_control() && c < 128 {
                self.search_term.push(c as u8 as char);
            } else {
                continue;
            }

            let ob = self.buffer_manager.active_mut();
            ob.cursor.cy = start_pos.row;
            ob.cursor.cx = start_pos.col;
            self.update_find_highlight();
            if !self.search_term.is_empty() && self.search_total_matches > 0 {
                self.find_next(1);
            }
        }

        self.search_active = false;
        self.set_status_message("");
        self.clear_find_highlight();
    }

    fn goto_line(&mut self) {
        let result = self.prompt("Aller a ligne,colonne: %s (Echap pour annuler)");
        let result = match result {
            Some(r) => r,
            None => return,
        };

        let trimmed = result.trim();
        let parts: Vec<&str> = trimmed.splitn(2, ',').collect();

        let line_str = parts[0].trim();
        if line_str.is_empty() {
            self.set_status_message_kind(
                "Numero de ligne requis (ligne ou ligne,colonne)",
                MessageKind::Error,
            );
            return;
        }

        let line_num: i64 = match line_str.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                self.set_status_message_kind("Numero de ligne invalide", MessageKind::Error);
                return;
            }
        };

        let col_num: Option<i64> = if parts.len() == 2 {
            let col_str = parts[1].trim();
            if col_str.is_empty() {
                None
            } else {
                match col_str.parse::<i64>() {
                    Ok(n) if n > 0 => Some(n),
                    _ => {
                        self.set_status_message_kind(
                            "Numero de colonne invalide",
                            MessageKind::Error,
                        );
                        return;
                    }
                }
            }
        } else {
            None
        };

        let ob = self.buffer_manager.active_mut();
        let mut target_row = (line_num - 1) as usize;
        if target_row >= ob.buffer.num_rows() {
            target_row = ob.buffer.num_rows().saturating_sub(1);
        }

        let row_size = ob.buffer.row(target_row).size();
        let target_col = match col_num {
            Some(c) => ((c - 1) as usize).min(row_size),
            None => 0,
        };

        ob.cursor.cy = target_row;
        ob.cursor.cx = target_col;
        ob.selection.is_active = false;
    }

    fn replace(&mut self) {
        let search_term = match self.prompt("Entre le mot a remplacer: %s (Echap pour annuler)") {
            Some(s) if !s.is_empty() => s,
            _ => return,
        };
        let replace_with = self
            .prompt("Remplacer par: %s (Echap pour annuler)")
            .unwrap_or_default();

        let num_rows = self.buffer_manager.active().buffer.num_rows();
        let mut replaced_count = 0i32;
        let mut replace_all = false;

        for r in 0..num_rows {
            let mut search_from_byte = 0usize;
            loop {
                let render = self
                    .buffer_manager
                    .active()
                    .buffer
                    .row(r)
                    .render()
                    .to_string();
                let pos = match utf8::find_pattern(&render, &search_term, search_from_byte) {
                    Some(p) => p,
                    None => break,
                };

                let char_idx = utf8::char_index_for_byte_offset(&render, pos);
                {
                    let ob = self.buffer_manager.active_mut();
                    ob.cursor.cy = r;
                    ob.cursor.cx = char_idx;
                }

                let do_replace = if replace_all {
                    true
                } else {
                    self.search_active = true;
                    self.search_term = search_term.clone();
                    self.update_find_highlight();

                    let confirm_msg = format!(
                        "Remplacer \"{}\" ? (o)ui / (n)on / (t)out / Echap pour arreter",
                        search_term
                    );
                    self.prompt_text = Some(confirm_msg);
                    self.refresh_screen();

                    let mut key = terminal::NO_KEY;
                    while key == terminal::NO_KEY {
                        key = self.terminal.read_key().unwrap_or(terminal::NO_KEY);
                        if key == terminal::NO_KEY {
                            self.apply_resize_if_needed();
                        }
                    }
                    self.search_active = false;
                    self.prompt_text = None;

                    if key == 0x1b {
                        self.clear_find_highlight();
                        self.set_status_message("Remplacement interrompu");
                        return;
                    }

                    if key == 't' as i32 || key == 'T' as i32 {
                        replace_all = true;
                        true
                    } else {
                        key == 'o' as i32 || key == 'O' as i32
                    }
                };

                if do_replace {
                    let char_count = utf8::codepoint_count(&search_term);
                    let start = Position::new(r, char_idx);
                    for _ in 0..char_count {
                        self.buffer_manager
                            .active_mut()
                            .buffer
                            .delete_char(r, char_idx + 1);
                    }
                    let end_pos = self.insert_text_at(start, &replace_with);

                    let ob = self.buffer_manager.active_mut();
                    let action = EditAction {
                        start,
                        removed_text: search_term.clone(),
                        inserted_text: replace_with.clone(),
                        cursor_before: start,
                        cursor_after: end_pos,
                        kind: EditKind::ReplaceBlock,
                    };
                    ob.undo_stack.push(action);

                    replaced_count += 1;
                    let ob = self.buffer_manager.active_mut();
                    ob.cursor.cx = end_pos.col;
                    let new_render = ob.buffer.row(r).render().to_string();
                    search_from_byte = utf8::byte_offset_for_char_index(&new_render, end_pos.col);
                    continue;
                }

                search_from_byte = pos + 1;
                self.buffer_manager.active_mut().cursor.cx = char_idx + 1;
            }
        }

        self.clear_find_highlight();
        let msg = format!("{} remplacement(s) effectue(s)", replaced_count);
        self.set_status_message_kind(msg, MessageKind::Success);
    }

    fn end_position_after_text(start: Position, text: &str) -> Position {
        let nl_count = text.chars().filter(|&c| c == '\n').count();
        if nl_count == 0 {
            return Position::new(start.row, start.col + utf8::codepoint_count(text));
        }
        let last_segment = text.rsplit('\n').next().unwrap_or("");
        Position::new(start.row + nl_count, utf8::codepoint_count(last_segment))
    }

    fn undo(&mut self) {
        let action = self.buffer_manager.active_mut().undo_stack.undo();
        let action = match action {
            Some(a) => a,
            None => {
                self.set_status_message("Rien a annuler");
                return;
            }
        };

        self.buffer_manager.active_mut().selection.is_active = false;

        if !action.inserted_text.is_empty() {
            let to = Self::end_position_after_text(action.start, &action.inserted_text);
            self.delete_selection_for_undo(action.start, to);
        }

        if !action.removed_text.is_empty() {
            self.insert_text_at(action.start, &action.removed_text);
        }

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = action.cursor_before.row;
        ob.cursor.cx = action.cursor_before.col;

        self.set_status_message("Annule");
    }

    fn redo(&mut self) {
        let action = self.buffer_manager.active_mut().undo_stack.redo();
        let action = match action {
            Some(a) => a,
            None => {
                self.set_status_message("Rien a retablir");
                return;
            }
        };

        self.buffer_manager.active_mut().selection.is_active = false;

        if !action.removed_text.is_empty() {
            let to = Self::end_position_after_text(action.start, &action.removed_text);
            self.delete_selection_for_undo(action.start, to);
        }

        if !action.inserted_text.is_empty() {
            self.insert_text_at(action.start, &action.inserted_text);
        }

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = action.cursor_after.row;
        ob.cursor.cx = action.cursor_after.col;

        self.set_status_message("Retabli");
    }

    fn copy_selection(&mut self) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if !has_selection {
            self.set_status_message("Aucune selection a copier");
            return;
        }
        let (start, end) = {
            let ob = self.buffer_manager.active();
            (ob.selection.start(), ob.selection.end())
        };
        let text = self.text_in_range(start, end);
        let ok = self.clipboard.copy(&text);
        self.set_status_message(if ok {
            "Copie dans le presse-papier systeme"
        } else {
            "Copie (presse-papier interne)"
        });
    }

    fn cut_selection(&mut self) {
        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if !has_selection {
            self.set_status_message("Aucune selection a couper");
            return;
        }
        let (start, end) = {
            let ob = self.buffer_manager.active();
            (ob.selection.start(), ob.selection.end())
        };
        let text = self.text_in_range(start, end);
        self.clipboard.copy(&text);
        self.delete_selection();
        self.set_status_message("Coupe");
    }

    fn paste_clipboard(&mut self) {
        let text = self.clipboard.paste();
        if text.is_empty() {
            self.set_status_message("Presse-papier vide");
            return;
        }

        let has_selection = {
            let ob = self.buffer_manager.active();
            ob.selection.is_active && !ob.selection.is_empty()
        };
        if has_selection {
            self.delete_selection();
        }

        let before = {
            let ob = self.buffer_manager.active();
            Position::new(ob.cursor.cy, ob.cursor.cx)
        };
        let after = self.insert_text_at(before, &text);

        let ob = self.buffer_manager.active_mut();
        ob.cursor.cy = after.row;
        ob.cursor.cx = after.col;

        let action = EditAction {
            start: before,
            inserted_text: text,
            cursor_before: before,
            cursor_after: after,
            kind: EditKind::InsertBlock,
            ..Default::default()
        };
        ob.undo_stack.push(action);

        self.set_status_message("Colle");
    }

    fn toggle_help(&mut self) {
        self.help_active = !self.help_active;
    }

    fn toggle_full_doc(&mut self) {
        self.overlay_full_doc = !self.overlay_full_doc;
    }

    fn handle_mouse_event(&mut self, ev: MouseEvent) {
        if ev.is_wheel {
            let ob = self.buffer_manager.active_mut();
            if ob.buffer.num_rows() == 0 {
                return;
            }
            let lines_per_step = 3i32;
            let delta = ev.wheel_delta * lines_per_step;

            let new_cy = if delta < 0 {
                ob.cursor.cy.saturating_sub((-delta) as usize)
            } else {
                (ob.cursor.cy + delta as usize).min(ob.buffer.num_rows() - 1)
            };

            ob.cursor.cy = new_cy;
            let row_len = ob.buffer.row(new_cy).size();
            if ob.cursor.cx > row_len {
                ob.cursor.cx = row_len;
            }
            ob.selection.is_active = false;
            return;
        }

        let tab_bar_height = if self.buffer_manager.count() > 1 {
            1
        } else {
            0
        };

        if tab_bar_height > 0 && ev.y == 0 && ev.pressed && !ev.is_drag {
            let mut col = 0i32;
            for i in 0..self.buffer_manager.count() {
                let other = self.buffer_manager.at(i);
                let mut name = other.buffer.filename().unwrap_or("[Sans nom]").to_string();
                if let Some(p) = name.rfind('/') {
                    name = name[p + 1..].to_string();
                }
                let mut label = format!(" {}", name);
                if other.buffer.dirty() {
                    label.push('*');
                }
                label.push(' ');
                let width = label.chars().count() as i32 + 2;
                if ev.x >= col && ev.x < col + width {
                    self.buffer_manager.switch_to(i);
                    return;
                }
                col += width;
            }
            return;
        }

        let search = SearchDisplayInfo::default();
        let state = self.build_view_state(search);
        let gutter = screen_renderer::gutter_width(&state) as i32;

        let screen_row = ev.y - tab_bar_height;
        let mut screen_col = ev.x - gutter;
        if screen_row < 0 {
            return;
        }
        if screen_col < 0 {
            screen_col = 0;
        }

        let ob = self.buffer_manager.active_mut();
        let mut filerow = (screen_row as usize) + ob.rowoff;
        if filerow >= ob.buffer.num_rows() {
            filerow = ob.buffer.num_rows().saturating_sub(1);
        }

        let rx = (screen_col as usize) + ob.coloff;
        let cx = if ob.buffer.num_rows() > 0 {
            ob.buffer.row(filerow).rx_to_cx(rx)
        } else {
            0
        };

        if ev.is_drag && ev.pressed {
            if !ob.selection.is_active {
                ob.selection.is_active = true;
                ob.selection.anchor = Position::new(ob.cursor.cy, ob.cursor.cx);
            }
            ob.cursor.cy = filerow;
            ob.cursor.cx = cx;
            ob.selection.active = Position::new(filerow, cx);
            return;
        }

        if ev.pressed {
            ob.cursor.cy = filerow;
            ob.cursor.cx = cx;
            ob.selection.is_active = true;
            ob.selection.anchor = Position::new(filerow, cx);
            ob.selection.active = Position::new(filerow, cx);
            return;
        }

        ob.cursor.cy = filerow;
        ob.cursor.cx = cx;
        if ob.selection.is_active && ob.selection.is_empty() {
            ob.selection.is_active = false;
        }
    }

    fn process_keypress(&mut self) {
        let c = match self.terminal.read_key() {
            Ok(k) => k,
            Err(_) => terminal::NO_KEY,
        };

        if c == terminal::NO_KEY {
            self.apply_resize_if_needed();
            return;
        }

        if self.overlay_full_doc {
            if c == 0x1b || c == ctrl('d') {
                self.overlay_full_doc = false;
            }
            return;
        }

        if c == terminal::MOUSE_EVENT {
            let ev = self.terminal.last_mouse_event();
            self.handle_mouse_event(ev);
            return;
        }

        match c {
            0x0d => self.insert_newline(),

            k if k == ctrl('q') => {
                if self.buffer_manager.any_dirty() && self.quit_times > 0 {
                    let msg = format!(
                        "ATTENTION ! Des modifications ne sont pas sauvegardees. Appuyez sur Ctrl-Q {} fois de plus pour quitter.",
                        self.quit_times
                    );
                    self.set_status_message_kind(msg, MessageKind::Error);
                    self.quit_times -= 1;
                    return;
                }
                self.terminal.write(b"\x1b[?1049l");
                std::process::exit(0);
            }

            k if k == ctrl('s') => self.save(),
            k if k == ctrl('f') => self.find(),
            k if k == ctrl('r') => self.replace(),
            k if k == ctrl('z') => self.undo(),
            k if k == ctrl('y') => self.redo(),
            k if k == ctrl('x') => self.cut_selection(),
            k if k == ctrl('c') => self.copy_selection(),
            k if k == ctrl('v') => self.paste_clipboard(),
            k if k == ctrl('l') => self.goto_line(),
            k if k == ctrl('n') => self.buffer_manager.next(),
            k if k == ctrl('p') => self.buffer_manager.previous(),
            k if k == ctrl('h') => self.toggle_help(),
            k if k == ctrl('d') => self.toggle_full_doc(),
            k if k == ctrl('k') => self.cut_current_line(),

            terminal::HOME_KEY => {
                let ob = self.buffer_manager.active_mut();
                ob.cursor.cx = 0;
                ob.selection.is_active = false;
            }

            terminal::END_KEY => {
                let ob = self.buffer_manager.active_mut();
                if ob.cursor.cy < ob.buffer.num_rows() {
                    ob.cursor.cx = ob.buffer.row(ob.cursor.cy).size();
                }
                ob.selection.is_active = false;
            }

            terminal::BACKSPACE => self.delete_char_backward(),
            terminal::DEL_KEY => self.delete_char_forward(),

            terminal::PAGE_UP | terminal::PAGE_DOWN => {
                let search = SearchDisplayInfo::default();
                let state = self.build_view_state(search);
                let text_rows = screen_renderer::text_area_screen_rows(&state);

                let ob = self.buffer_manager.active_mut();
                let last_row = ob.buffer.num_rows().saturating_sub(1);
                if c == terminal::PAGE_UP {
                    ob.cursor.cy = ob.rowoff.min(last_row);
                } else {
                    ob.cursor.cy = (ob.rowoff + text_rows.saturating_sub(1)).min(last_row);
                }

                let dir = if c == terminal::PAGE_UP {
                    terminal::ARROW_UP
                } else {
                    terminal::ARROW_DOWN
                };
                for _ in 0..text_rows {
                    self.move_cursor(dir, false);
                }
            }

            terminal::ARROW_UP
            | terminal::ARROW_DOWN
            | terminal::ARROW_LEFT
            | terminal::ARROW_RIGHT => {
                self.move_cursor(c, false);
            }

            terminal::CTRL_ARROW_UP => self.move_cursor(terminal::ARROW_UP, true),
            terminal::CTRL_ARROW_DOWN => self.move_cursor(terminal::ARROW_DOWN, true),
            terminal::CTRL_ARROW_LEFT => self.move_cursor(terminal::ARROW_LEFT, true),
            terminal::CTRL_ARROW_RIGHT => self.move_cursor(terminal::ARROW_RIGHT, true),

            0x1b => {
                self.buffer_manager.active_mut().selection.is_active = false;
            }

            9 => self.insert_codepoint('\t'),

            _ => {
                if terminal::is_encoded_codepoint(c) {
                    self.insert_codepoint(terminal::decode_codepoint(c));
                } else if !(c as u8 as char).is_control() && c < 128 {
                    self.insert_codepoint(c as u8 as char);
                }
            }
        }

        self.quit_times = self.config.quit_times;
    }
}
