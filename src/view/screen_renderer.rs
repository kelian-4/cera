use crate::editor::buffer_manager::BufferManager;
use crate::io::config::Config;
use crate::model::selection::Position;
use crate::view::colors::{bg_sequence, fg_sequence, reset_fg_sequence};
use crate::view::help_text::{full_documentation, MessageKind};

#[derive(Clone, Copy, Default)]
pub struct SearchDisplayInfo {
    pub active: bool,
    pub current_index: usize,
    pub total_matches: usize,
}

pub struct ViewState<'a> {
    pub buffer_manager: &'a BufferManager,
    pub rowoff: usize,
    pub coloff: usize,
    pub screenrows: usize,
    pub screencols: usize,
    pub status_message: String,
    pub status_message_kind: MessageKind,
    pub status_message_age_seconds: i64,
    pub overlay_full_doc: bool,
    pub help_active: bool,
    pub prompt_text: Option<String>,
    pub search: SearchDisplayInfo,
    pub search_term: String,
    pub config: &'a Config,
}

pub struct AppendBuffer {
    data: String,
}

impl AppendBuffer {
    pub fn new() -> Self {
        AppendBuffer {
            data: String::new(),
        }
    }

    pub fn append(&mut self, s: &str) {
        self.data.push_str(s);
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

impl Default for AppendBuffer {
    fn default() -> Self {
        Self::new()
    }
}

fn digit_count(n: usize) -> usize {
    if n < 10 {
        1
    } else {
        let mut count = 0;
        let mut v = n;
        while v > 0 {
            count += 1;
            v /= 10;
        }
        count
    }
}

pub fn gutter_width(state: &ViewState) -> usize {
    if !state.config.show_line_numbers {
        return 0;
    }
    let buf = &state.buffer_manager.active().buffer;
    digit_count(buf.num_rows().max(1)) + 2
}

pub fn text_area_screen_rows(state: &ViewState) -> usize {
    let mut reserved = 2usize;
    if state.buffer_manager.count() > 1 {
        reserved += 1;
    }
    if state.help_active {
        reserved += 2;
    }
    state.screenrows.saturating_sub(reserved).max(1)
}

fn draw_tab_bar(ab: &mut AppendBuffer, state: &ViewState) {
    if state.buffer_manager.count() <= 1 {
        return;
    }

    let mut line = String::new();
    for i in 0..state.buffer_manager.count() {
        let ob = state.buffer_manager.at(i);
        let mut name = ob.buffer.filename().unwrap_or("[Sans nom]").to_string();
        if let Some(pos) = name.rfind('/') {
            name = name[pos + 1..].to_string();
        }

        let mut label = format!(" {}", name);
        if ob.buffer.dirty() {
            label.push('*');
        }
        label.push(' ');

        if i == state.buffer_manager.active_index() {
            line.push_str(&format!("[{}]", label));
        } else {
            line.push_str(&format!(" {} ", label));
        }
    }

    let line_chars: Vec<char> = line.chars().collect();
    let truncated: String = if line_chars.len() > state.screencols {
        line_chars[..state.screencols].iter().collect()
    } else {
        line
    };
    ab.append(&truncated);
    for _ in truncated.chars().count()..state.screencols {
        ab.append(" ");
    }
    ab.append("\r\n");
}

fn draw_rows(ab: &mut AppendBuffer, state: &ViewState) {
    let ob = state.buffer_manager.active();
    let buffer = &ob.buffer;
    let text_rows = text_area_screen_rows(state);
    let gutter = gutter_width(state);
    let text_cols = state.screencols.saturating_sub(gutter).max(1);

    let true_color = state.config.true_color;
    let theme = &state.config.theme;

    let has_selection = ob.selection.is_active && !ob.selection.is_empty();
    let (sel_start, sel_end) = if has_selection {
        (ob.selection.start(), ob.selection.end())
    } else {
        (Position::default(), Position::default())
    };

    for y in 0..text_rows {
        let filerow = y + state.rowoff;

        if gutter > 0 {
            ab.append(&fg_sequence(theme.line_number_fg, true_color));
            if filerow < buffer.num_rows() {
                let numstr = format!("{:>width$} ", filerow + 1, width = gutter - 1);
                ab.append(&numstr);
            } else {
                ab.append(&" ".repeat(gutter));
            }
            ab.append(reset_fg_sequence());
        }

        if filerow >= buffer.num_rows() {
            if buffer.num_rows() == 0 && y == text_rows / 3 {
                let welcome = "Cera editor -- version 0.1.0";
                let welcome_len = welcome.chars().count().min(text_cols);
                let welcome: String = welcome.chars().take(welcome_len).collect();
                let padding = (text_cols.saturating_sub(welcome_len)) / 2;
                for _ in 0..padding {
                    ab.append(" ");
                }
                ab.append(&welcome);
            }
        } else {
            let row = buffer.row(filerow);
            let render_chars: Vec<char> = row.render().chars().collect();
            let hl = row.hl();

            let mut col_width_used = 0usize;
            let mut current_color_rgb: Option<u32> = None;
            let mut in_selection_bg = false;

            let mut ci = state.coloff;
            while ci < row.rsize() && col_width_used < text_cols {
                let char_width = row.render_char_width(ci);
                if col_width_used + char_width > text_cols {
                    break;
                }

                let mut selected = false;
                if has_selection && filerow >= sel_start.row && filerow <= sel_end.row {
                    selected = if filerow == sel_start.row && filerow == sel_end.row {
                        ci >= sel_start.col && ci < sel_end.col
                    } else if filerow == sel_start.row {
                        ci >= sel_start.col
                    } else if filerow == sel_end.row {
                        ci < sel_end.col
                    } else {
                        true
                    };
                }

                if selected != in_selection_bg {
                    if selected {
                        ab.append(&bg_sequence(theme.selection_bg, true_color));
                    } else {
                        ab.append(crate::view::colors::reset_bg_sequence());
                    }
                    in_selection_bg = selected;
                }

                let ch = render_chars.get(ci).copied().unwrap_or(' ');

                if (ch as u32) < 32 && ch != '\0' {
                    let sym = char::from_u32('@' as u32 + ch as u32).unwrap_or('?');
                    ab.append("\x1b[7m");
                    ab.append(&sym.to_string());
                    ab.append("\x1b[m");
                    if let Some(rgb) = current_color_rgb {
                        ab.append(&fg_sequence(rgb, true_color));
                    }
                } else {
                    let h = hl
                        .get(ci)
                        .copied()
                        .unwrap_or(crate::model::row::Highlight::Normal);
                    let rgb = crate::view::colors::highlight_to_rgb(h, theme);
                    if current_color_rgb != Some(rgb) {
                        current_color_rgb = Some(rgb);
                        ab.append(&fg_sequence(rgb, true_color));
                    }
                    ab.append(&ch.to_string());
                }

                col_width_used += char_width;
                ci += 1;
            }

            ab.append(reset_fg_sequence());
            if in_selection_bg {
                ab.append(crate::view::colors::reset_bg_sequence());
            }
        }

        ab.append("\x1b[K");
        ab.append("\r\n");
    }
}

fn draw_help_bar(ab: &mut AppendBuffer, state: &ViewState) {
    if !state.help_active {
        return;
    }

    let entries = crate::view::help_text::short_help_entries();
    let min_column_width = 13usize;
    let columns_per_line = (state.screencols / min_column_width).max(1);
    let column_width = state.screencols / columns_per_line;
    let text_width = column_width.saturating_sub(1).max(1);

    let mut lines: Vec<String> = vec![String::new(), String::new()];
    let max_entries = columns_per_line * 2;

    for (i, entry) in entries.iter().take(max_entries).enumerate() {
        let line_idx = i % 2;
        let cell = format!("{} {}", entry.key, entry.label);

        let remaining = state
            .screencols
            .saturating_sub(lines[line_idx].chars().count());
        if remaining == 0 {
            continue;
        }

        let cell_len = cell.chars().count();
        let text_part: String = if cell_len > text_width {
            cell.chars().take(text_width).collect()
        } else {
            cell.clone()
        };

        let mut cell_final = text_part;
        let total_width = column_width.min(remaining);
        while cell_final.chars().count() < total_width {
            cell_final.push(' ');
        }
        if cell_final.chars().count() > remaining {
            cell_final = cell_final.chars().take(remaining).collect();
        }

        lines[line_idx].push_str(&cell_final);
    }

    for line in lines.iter().take(2) {
        let line_len = line.chars().count().min(state.screencols);
        let line_trunc: String = line.chars().take(line_len).collect();
        ab.append(&line_trunc);
        for _ in line_len..state.screencols {
            ab.append(" ");
        }
        ab.append("\r\n");
    }
}

fn draw_status_bar(ab: &mut AppendBuffer, state: &ViewState) {
    if let Some(prompt_text) = &state.prompt_text {
        let line = format!(" {}", prompt_text);
        let line_len = line.chars().count().min(state.screencols);
        let line_trunc: String = line.chars().take(line_len).collect();
        ab.append(&line_trunc);
        for _ in line_len..state.screencols {
            ab.append(" ");
        }
        ab.append("\r\n");
        return;
    }

    let ob = state.buffer_manager.active();
    let buffer = &ob.buffer;

    let left = if state.search.active {
        format!(
            " recherche: {} ({}/{})",
            state.search_term, state.search.current_index, state.search.total_matches
        )
    } else {
        " ^H aide".to_string()
    };

    let mut filename = buffer.filename().unwrap_or("[Sans nom]").to_string();
    if let Some(pos) = filename.rfind('/') {
        filename = filename[pos + 1..].to_string();
    }
    let mut center = filename;
    if buffer.dirty() {
        center.push_str(" [modifie]");
    }

    let total_lines = buffer.num_rows().max(1);
    let percent = ((ob.cursor.cy + 1) * 100) / total_lines;
    let bar_width = 10usize;
    let filled = (percent * bar_width) / 100;
    let mut bar = String::with_capacity(bar_width);
    for i in 0..bar_width {
        bar.push(if i < filled { '█' } else { '░' });
    }
    let right = format!("{} {}/{} ", bar, ob.cursor.cy + 1, total_lines);

    let total = state.screencols;
    let left_len = left.chars().count().min(total);
    let left_trunc: String = left.chars().take(left_len).collect();
    ab.append(&left_trunc);

    let right_len = right.chars().count();
    let center_len = center.chars().count();

    let remaining = total.saturating_sub(left_len + right_len);
    let center_start = remaining.saturating_sub(center_len) / 2;

    let mut written = left_len;
    let mut i = 0;
    while i < center_start && written < total.saturating_sub(right_len) {
        ab.append(" ");
        written += 1;
        i += 1;
    }

    let center_write = center_len.min(total.saturating_sub(right_len).saturating_sub(written));
    let center_trunc: String = center.chars().take(center_write).collect();
    ab.append(&center_trunc);
    written += center_write;

    while written < total.saturating_sub(right_len) {
        ab.append(" ");
        written += 1;
    }

    if right_len > 0 && total.saturating_sub(written) >= right_len {
        ab.append(&right);
        written += right_len;
    }
    while written < total {
        ab.append(" ");
        written += 1;
    }

    ab.append("\r\n");
}

fn draw_message_bar(ab: &mut AppendBuffer, state: &ViewState) {
    ab.append("\x1b[K");
    let msg_len = state.status_message.chars().count().min(state.screencols);
    if msg_len > 0 && state.status_message_age_seconds < 5 {
        let color = match state.status_message_kind {
            MessageKind::Success => state.config.theme.success_fg,
            MessageKind::Error => state.config.theme.error_fg,
            MessageKind::Info => state.config.theme.normal,
        };
        ab.append(&fg_sequence(color, state.config.true_color));
        let msg_trunc: String = state.status_message.chars().take(msg_len).collect();
        ab.append(&msg_trunc);
        ab.append(reset_fg_sequence());
    }
}

fn draw_full_doc(ab: &mut AppendBuffer, state: &ViewState) {
    let theme = &state.config.theme;
    let true_color = state.config.true_color;

    ab.append(&fg_sequence(theme.keyword1, true_color));
    let title = " Documentation de Cera -- Echap pour revenir au fichier ";
    let title_len = title.chars().count().min(state.screencols);
    let title_trunc: String = title.chars().take(title_len).collect();
    ab.append(&title_trunc);
    ab.append(reset_fg_sequence());
    ab.append("\x1b[K");
    ab.append("\r\n");

    let mut rows_used = 1usize;
    for cat in full_documentation() {
        if rows_used >= state.screenrows.saturating_sub(1) {
            break;
        }
        ab.append(&fg_sequence(theme.keyword1, true_color));
        ab.append(cat.name);
        ab.append(reset_fg_sequence());
        ab.append("\x1b[K");
        ab.append("\r\n");
        rows_used += 1;

        for entry in cat.entries {
            if rows_used >= state.screenrows.saturating_sub(1) {
                break;
            }
            let line = format!("  {:<14} {}", entry.shortcut, entry.description);
            let line_len = line.chars().count().min(state.screencols);
            let line_trunc: String = line.chars().take(line_len).collect();
            ab.append(&line_trunc);
            ab.append("\x1b[K");
            ab.append("\r\n");
            rows_used += 1;
        }

        if rows_used < state.screenrows.saturating_sub(1) {
            ab.append("\x1b[K");
            ab.append("\r\n");
            rows_used += 1;
        }
    }

    while rows_used < state.screenrows {
        ab.append("\x1b[K");
        ab.append("\r\n");
        rows_used += 1;
    }

    ab.append("\x1b[K");
}

pub fn refresh(state: &ViewState) -> String {
    let mut ab = AppendBuffer::new();

    ab.append("\x1b[?25l");
    ab.append("\x1b[H");

    if state.overlay_full_doc {
        draw_full_doc(&mut ab, state);
        ab.append("\x1b[?25h");
        return ab.data().to_string();
    }

    draw_tab_bar(&mut ab, state);
    draw_rows(&mut ab, state);
    draw_status_bar(&mut ab, state);
    draw_help_bar(&mut ab, state);
    draw_message_bar(&mut ab, state);

    let ob = state.buffer_manager.active();
    let gutter = gutter_width(state);
    let tab_bar_height = if state.buffer_manager.count() > 1 {
        1
    } else {
        0
    };

    let cursor_row = tab_bar_height + (ob.cursor.cy.saturating_sub(state.rowoff)) + 1;
    let cursor_col = gutter + (ob.cursor.rx.saturating_sub(state.coloff)) + 1;
    ab.append(&format!("\x1b[{};{}H", cursor_row, cursor_col));

    ab.append("\x1b[?25h");

    ab.data().to_string()
}
