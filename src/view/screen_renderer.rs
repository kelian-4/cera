use crate::editor::buffer_manager::BufferManager;
use crate::io::config::Config;
use crate::model::selection::Position;
use crate::view::colors::{bg_sequence, fg_sequence, reset_fg_sequence};
use crate::view::help_text::{
    clamp_doc_scroll, doc_visible_rows, flatten_documentation, full_documentation, DocLine,
    MessageKind,
};

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
    pub doc_scroll: usize,
    pub help_active: bool,
    pub prompt_text: Option<String>,
    pub prompt_cursor_offset: Option<usize>,
    pub search: SearchDisplayInfo,
    pub search_term: String,
    pub search_cursor: usize,
    pub config: &'a Config,
    pub i18n: &'a crate::i18n::Translations,
    pub plugins: &'a [crate::plugin::registry::PluginManifest],
    pub plugin_view: Option<&'a [crate::plugin::protocol::RenderLine]>,
    pub side_panel: Option<(&'a [crate::plugin::protocol::RenderLine], usize, bool)>,
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

pub fn wrap_message_two_lines(text: &str, width: usize) -> (String, Option<String>) {
    if width == 0 {
        return (String::new(), None);
    }
    let char_count = text.chars().count();
    if char_count <= width {
        return (text.to_string(), None);
    }

    let chars: Vec<char> = text.chars().collect();

    let mut split_at = width;
    if let Some(last_space) = chars[..width].iter().rposition(|&c| c == ' ') {
        if last_space > 0 {
            split_at = last_space;
        }
    }

    let first_line: String = chars[..split_at].iter().collect();
    let mut rest_start = split_at;
    while rest_start < chars.len() && chars[rest_start] == ' ' {
        rest_start += 1;
    }

    let remaining = &chars[rest_start..];
    let second_len = remaining.len().min(width);
    let second_line: String = remaining[..second_len].iter().collect();

    (first_line, Some(second_line))
}

pub fn gutter_width(state: &ViewState) -> usize {
    if !state.config.show_line_numbers {
        return 0;
    }
    let buf = &state.buffer_manager.active().buffer;
    digit_count(buf.num_rows().max(1)) + 2
}

pub fn side_panel_width(state: &ViewState) -> usize {
    state.side_panel.map(|(_, width, _)| width + 1).unwrap_or(0)
}

pub fn text_area_screen_cols(state: &ViewState) -> usize {
    state
        .screencols
        .saturating_sub(gutter_width(state))
        .saturating_sub(side_panel_width(state))
        .max(1)
}

pub fn tab_bar_height(state: &ViewState) -> usize {
    if state.buffer_manager.count() > 1 || state.side_panel.is_some() {
        1
    } else {
        0
    }
}

pub fn text_area_screen_rows(state: &ViewState) -> usize {
    let mut reserved = status_bar_rows(state) + message_bar_rows(state) + tab_bar_height(state);
    if state.help_active {
        reserved += 2;
    }
    state.screenrows.saturating_sub(reserved).max(1)
}

fn status_bar_rows(state: &ViewState) -> usize {
    match &state.prompt_text {
        Some(prompt_text) => {
            let line = format!(" {}", prompt_text);
            let (_, second) = wrap_message_two_lines(&line, state.screencols);
            if second.is_some() {
                2
            } else {
                1
            }
        }
        None => 1,
    }
}

fn message_bar_rows(state: &ViewState) -> usize {
    if state.status_message.is_empty() || state.status_message_age_seconds >= 5 {
        return 1;
    }
    let (_, second) = wrap_message_two_lines(&state.status_message, state.screencols);
    if second.is_some() {
        2
    } else {
        1
    }
}

fn draw_tab_bar(ab: &mut AppendBuffer, state: &ViewState) {
    if state.buffer_manager.count() <= 1 {
        if let Some((panel_lines, width, _)) = state.side_panel {
            draw_panel_row(ab, state, panel_lines, 0, width);
            ab.append("\x1b[K");
            ab.append("\r\n");
        }
        return;
    }

    let tabs_width = state.screencols.saturating_sub(side_panel_width(state));

    if let Some((panel_lines, width, _)) = state.side_panel {
        draw_panel_row(ab, state, panel_lines, 0, width);
    }

    let mut line = String::new();
    for i in 0..state.buffer_manager.count() {
        let ob = state.buffer_manager.at(i);
        let mut name = crate::view::sanitize::neutralize_controls(
            ob.buffer.filename().unwrap_or("[Sans nom]"),
        );
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
    let truncated: String = if line_chars.len() > tabs_width {
        line_chars[..tabs_width].iter().collect()
    } else {
        line
    };
    ab.append(&truncated);
    for _ in truncated.chars().count()..tabs_width {
        ab.append(" ");
    }
    ab.append("\r\n");
}

fn draw_panel_row(
    ab: &mut AppendBuffer,
    state: &ViewState,
    panel_lines: &[crate::plugin::protocol::RenderLine],
    y: usize,
    width: usize,
) {
    let theme = &state.config.theme;
    let true_color = state.config.true_color;

    let mut written = 0usize;
    if let Some(line) = panel_lines.get(y) {
        for segment in &line.segments {
            if written >= width {
                break;
            }
            let color = segment.color.unwrap_or(theme.normal);
            ab.append(&fg_sequence(color, true_color));
            if segment.bold {
                ab.append("\x1b[1m");
            }
            let remaining = width - written;
            let text_len = segment.text.chars().count().min(remaining);
            let text_trunc: String = segment.text.chars().take(text_len).collect();
            ab.append(&text_trunc);
            if segment.bold {
                ab.append("\x1b[22m");
            }
            ab.append(reset_fg_sequence());
            written += text_len;
        }
    }
    for _ in written..width {
        ab.append(" ");
    }
    ab.append(&fg_sequence(theme.comment, true_color));
    ab.append("\u{2502}");
    ab.append(reset_fg_sequence());
}

fn draw_rows(ab: &mut AppendBuffer, state: &ViewState) {
    let ob = state.buffer_manager.active();
    let buffer = &ob.buffer;
    let text_rows = text_area_screen_rows(state);
    let gutter = gutter_width(state);
    let text_cols = text_area_screen_cols(state);

    let true_color = state.config.true_color;
    let theme = &state.config.theme;

    let has_selection = ob.selection.is_active && !ob.selection.is_empty();
    let (sel_start, sel_end) = if has_selection {
        (ob.selection.start(), ob.selection.end())
    } else {
        (Position::default(), Position::default())
    };

    let guide_unit = if state.config.indent_guides {
        let filetype = ob
            .active_syntax_idx
            .and_then(|i| crate::syntax::database().get(i))
            .map(|s| s.filetype)
            .unwrap_or("");
        crate::view::indent_guides::detect_unit(buffer, filetype, crate::model::row::tab_stop())
    } else {
        0
    };

    for y in 0..text_rows {
        let filerow = y + state.rowoff;

        if let Some((panel_lines, width, _)) = state.side_panel {
            draw_panel_row(ab, state, panel_lines, y + 1, width);
        }

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
            let is_single_empty_line = buffer.num_rows() == 1 && buffer.row(0).chars().is_empty();
            if is_single_empty_line && y == text_rows / 3 {
                let welcome = state.i18n.t("ui.welcome_message");
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

            let (guide_indent, row_is_blank) = if guide_unit > 0 {
                (
                    crate::view::indent_guides::effective_indent(buffer, filerow),
                    crate::view::indent_guides::row_indent(buffer, filerow).is_none(),
                )
            } else {
                (0, false)
            };

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

                if ch != '\0' && ch.is_control() {
                    let code = ch as u32;
                    let sym = if code < 32 {
                        char::from_u32('@' as u32 + code).unwrap_or('?')
                    } else {
                        '?'
                    };
                    ab.append("\x1b[7m");
                    ab.append(&sym.to_string());
                    ab.append("\x1b[m");
                    if let Some(rgb) = current_color_rgb {
                        ab.append(&fg_sequence(rgb, true_color));
                    }
                } else if ch == ' '
                    && crate::view::indent_guides::is_guide_col(ci, guide_indent, guide_unit)
                {
                    current_color_rgb = Some(theme.line_number_fg);
                    ab.append(&fg_sequence(theme.line_number_fg, true_color));
                    ab.append("│");
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

            if row_is_blank && ci >= row.rsize() {
                while ci < guide_indent && col_width_used < text_cols {
                    if ci >= state.coloff
                        && crate::view::indent_guides::is_guide_col(ci, guide_indent, guide_unit)
                    {
                        ab.append(&fg_sequence(theme.line_number_fg, true_color));
                        ab.append("│");
                    } else if ci >= state.coloff {
                        ab.append(" ");
                    } else {
                        ci += 1;
                        continue;
                    }
                    col_width_used += 1;
                    ci += 1;
                }
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

    let entries = crate::view::help_text::short_help_entries(state.i18n);
    let min_gap = 2usize;

    let pairs: Vec<(
        &crate::view::help_text::ShortHelpEntry,
        Option<&crate::view::help_text::ShortHelpEntry>,
    )> = entries.chunks(2).map(|c| (&c[0], c.get(1))).collect();

    let natural_widths: Vec<usize> = pairs
        .iter()
        .map(|(top, bottom)| {
            let top_len = format!("{} {}", top.key, top.label).chars().count();
            let bottom_len = bottom
                .map(|e| format!("{} {}", e.key, e.label).chars().count())
                .unwrap_or(0);
            top_len.max(bottom_len)
        })
        .collect();

    let mut num_columns = 0usize;
    let mut used = 0usize;
    for (i, &w) in natural_widths.iter().enumerate() {
        let extra = if i == 0 { w } else { min_gap + w };
        if used + extra > state.screencols {
            break;
        }
        used += extra;
        num_columns = i + 1;
    }
    let num_columns = num_columns.max(1);

    let top_cells: Vec<String> = pairs
        .iter()
        .take(num_columns)
        .map(|(top, _)| format!("{} {}", top.key, top.label))
        .collect();
    let bottom_cells: Vec<String> = pairs
        .iter()
        .take(num_columns)
        .map(|(_, bottom)| {
            bottom
                .map(|e| format!("{} {}", e.key, e.label))
                .unwrap_or_default()
        })
        .collect();
    let column_widths: Vec<usize> = natural_widths.into_iter().take(num_columns).collect();

    draw_help_grid_line(ab, state.screencols, &top_cells, &column_widths);
    draw_help_grid_line(ab, state.screencols, &bottom_cells, &column_widths);
}

fn draw_help_grid_line(
    ab: &mut AppendBuffer,
    width: usize,
    cells: &[String],
    column_widths: &[usize],
) {
    if cells.is_empty() {
        for _ in 0..width {
            ab.append(" ");
        }
        ab.append("\r\n");
        return;
    }

    let total_text_len: usize = column_widths.iter().sum();

    if cells.len() == 1 {
        let cell = &cells[0];
        let cell_len = cell.chars().count();
        ab.append(cell);
        for _ in cell_len..width {
            ab.append(" ");
        }
        ab.append("\r\n");
        return;
    }

    let gaps = cells.len() - 1;
    let available_space = width.saturating_sub(total_text_len);
    let base_gap = available_space / gaps;
    let extra = available_space % gaps;

    let mut written = 0usize;
    for (i, cell) in cells.iter().enumerate() {
        if written >= width {
            break;
        }
        let col_width = column_widths[i];
        let cell_len = cell.chars().count().min(width - written);
        let cell_trunc: String = cell.chars().take(cell_len).collect();
        ab.append(&cell_trunc);
        written += cell_len;

        let pad_to_col = col_width
            .saturating_sub(cell_len)
            .min(width.saturating_sub(written));
        for _ in 0..pad_to_col {
            ab.append(" ");
        }
        written += pad_to_col;

        if i < gaps {
            let mut gap = base_gap;
            if i < extra {
                gap += 1;
            }
            gap = gap.min(width.saturating_sub(written));
            for _ in 0..gap {
                ab.append(" ");
            }
            written += gap;
        }
    }

    for _ in written..width {
        ab.append(" ");
    }
    ab.append("\r\n");
}

fn append_with_custom_cursor(
    ab: &mut AppendBuffer,
    text: &str,
    cursor_offset: Option<usize>,
    max_width: usize,
) -> usize {
    let chars: Vec<char> = text.chars().collect();
    let at_end_but_full = matches!(cursor_offset, Some(o) if o >= chars.len())
        && chars.len() >= max_width
        && !chars.is_empty();

    let effective_offset = if at_end_but_full {
        Some(chars.len() - 1)
    } else {
        cursor_offset
    };

    match effective_offset {
        None => {
            ab.append(text);
            chars.len()
        }
        Some(offset) if offset < chars.len() => {
            let before: String = chars[..offset].iter().collect();
            let cursor_char = chars[offset];
            let after: String = chars[offset + 1..].iter().collect();
            ab.append(&before);
            ab.append("\x1b[7m");
            ab.append(&cursor_char.to_string());
            ab.append("\x1b[27m");
            ab.append(&after);
            chars.len()
        }
        Some(_) => {
            ab.append(text);
            ab.append("\x1b[7m \x1b[27m");
            chars.len() + 1
        }
    }
}

fn draw_status_bar(ab: &mut AppendBuffer, state: &ViewState) {
    if let Some(prompt_text) = &state.prompt_text {
        let line = format!(" {}", prompt_text);
        let (first, second) = wrap_message_two_lines(&line, state.screencols);

        let first_len = first.chars().count();
        let line_total = line.chars().count();
        let mut first_cursor = None;
        let mut second_cursor = None;
        if let Some(offset) = state.prompt_cursor_offset {
            let target = (1 + offset).min(line_total);
            if target <= first_len {
                first_cursor = Some(target);
            } else if second.is_some() {
                second_cursor = Some(target - first_len);
            } else {
                first_cursor = Some(first_len);
            }
        }

        let written = append_with_custom_cursor(ab, &first, first_cursor, state.screencols);
        for _ in written..state.screencols {
            ab.append(" ");
        }
        ab.append("\r\n");

        if let Some(second) = second {
            let written2 = append_with_custom_cursor(ab, &second, second_cursor, state.screencols);
            for _ in written2..state.screencols {
                ab.append(" ");
            }
            ab.append("\r\n");
        }
        return;
    }

    let ob = state.buffer_manager.active();
    let buffer = &ob.buffer;

    let search_prefix = format!(" {}", state.i18n.t("search.prefix"));
    let left = if state.search.active {
        format!(
            "{}{} ({}/{})",
            search_prefix,
            state.search_term,
            state.search.current_index,
            state.search.total_matches
        )
    } else {
        format!(" {}", state.i18n.t("help.toggle_hint"))
    };

    let no_name = state.i18n.t("doc.no_filename");
    let mut filename =
        crate::view::sanitize::neutralize_controls(buffer.filename().unwrap_or(&no_name));
    if let Some(pos) = filename.rfind('/') {
        filename = filename[pos + 1..].to_string();
    }
    let mut center = filename;
    if buffer.dirty() {
        center.push(' ');
        center.push_str(&state.i18n.t("doc.modified_suffix"));
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

    let left_cursor = if state.search.active {
        let prefix_len = search_prefix.chars().count();
        Some((prefix_len + state.search_cursor).min(left_len))
    } else {
        None
    };
    append_with_custom_cursor(ab, &left_trunc, left_cursor, left_len);

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
    if !state.status_message.is_empty() && state.status_message_age_seconds < 5 {
        let color = match state.status_message_kind {
            MessageKind::Success => state.config.theme.success_fg,
            MessageKind::Error => state.config.theme.error_fg,
            MessageKind::Info => state.config.theme.normal,
        };
        let (first, second) = wrap_message_two_lines(&state.status_message, state.screencols);

        ab.append(&fg_sequence(color, state.config.true_color));
        ab.append(&first);
        ab.append(reset_fg_sequence());

        if let Some(second) = second {
            ab.append("\r\n");
            ab.append("\x1b[K");
            ab.append(&fg_sequence(color, state.config.true_color));
            ab.append(&second);
            ab.append(reset_fg_sequence());
        }
    }
}

fn draw_full_doc(ab: &mut AppendBuffer, state: &ViewState) {
    let theme = &state.config.theme;
    let true_color = state.config.true_color;

    ab.append(&fg_sequence(theme.keyword1, true_color));
    let title = state.i18n.t("doc.title");
    let title_len = title.chars().count().min(state.screencols);
    let title_trunc: String = title.chars().take(title_len).collect();
    ab.append(&title_trunc);
    ab.append(reset_fg_sequence());
    ab.append("\x1b[K");
    ab.append("\r\n");

    let mut rows_used = 1usize;
    let lines = flatten_documentation(&full_documentation(state.i18n, state.plugins));
    let visible = doc_visible_rows(state.screenrows);
    let scroll = clamp_doc_scroll(state.doc_scroll, lines.len(), visible);
    for line in lines.iter().skip(scroll).take(visible) {
        match line {
            DocLine::Header(name) => {
                ab.append(&fg_sequence(theme.keyword1, true_color));
                ab.append(name);
                ab.append(reset_fg_sequence());
            }
            DocLine::Entry {
                shortcut,
                description,
            } => {
                let text = format!("  {:<14} {}", shortcut, description);
                let text_len = text.chars().count().min(state.screencols);
                let text_trunc: String = text.chars().take(text_len).collect();
                ab.append(&text_trunc);
            }
            DocLine::Blank => {}
        }
        ab.append("\x1b[K");
        ab.append("\r\n");
        rows_used += 1;
    }

    while rows_used < state.screenrows {
        ab.append("\x1b[K");
        ab.append("\r\n");
        rows_used += 1;
    }

    ab.append("\x1b[K");
}

fn draw_plugin_view(
    ab: &mut AppendBuffer,
    state: &ViewState,
    lines: &[crate::plugin::protocol::RenderLine],
) {
    let theme = &state.config.theme;
    let true_color = state.config.true_color;
    let mut rows_used = 0usize;

    for line in lines {
        if rows_used >= state.screenrows {
            break;
        }
        let mut written = 0usize;
        for segment in &line.segments {
            if written >= state.screencols {
                break;
            }
            let color = segment.color.unwrap_or(theme.normal);
            ab.append(&fg_sequence(color, true_color));
            if segment.bold {
                ab.append("\x1b[1m");
            }
            let remaining = state.screencols - written;
            let text_len = segment.text.chars().count().min(remaining);
            let text_trunc: String = segment.text.chars().take(text_len).collect();
            ab.append(&text_trunc);
            if segment.bold {
                ab.append("\x1b[22m");
            }
            ab.append(reset_fg_sequence());
            written += text_len;
        }
        ab.append("\x1b[K");
        ab.append("\r\n");
        rows_used += 1;
    }

    for _ in rows_used..state.screenrows {
        ab.append("\x1b[K");
        ab.append("\r\n");
    }
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

    if let Some(lines) = state.plugin_view {
        draw_plugin_view(&mut ab, state, lines);
        ab.append("\x1b[?25l");
        return ab.data().to_string();
    }

    draw_tab_bar(&mut ab, state);
    draw_rows(&mut ab, state);
    draw_status_bar(&mut ab, state);
    draw_help_bar(&mut ab, state);
    draw_message_bar(&mut ab, state);

    let ob = state.buffer_manager.active();
    let gutter = gutter_width(state);
    let panel_width = side_panel_width(state);
    let tab_bar_height_val = tab_bar_height(state);

    let panel_focused = state
        .side_panel
        .map(|(_, _, focused)| focused)
        .unwrap_or(false);
    if panel_focused {
        ab.append("\x1b[?25l");
        return ab.data().to_string();
    }

    let cursor_row = tab_bar_height_val + (ob.cursor.cy.saturating_sub(state.rowoff)) + 1;
    let cursor_col = panel_width + gutter + (ob.cursor.rx.saturating_sub(state.coloff)) + 1;
    ab.append(&format!("\x1b[{};{}H", cursor_row, cursor_col));

    ab.append("\x1b[?25h");

    ab.data().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_message_fits_on_one_line() {
        let (first, second) = wrap_message_two_lines("short message", 40);
        assert_eq!(first, "short message");
        assert_eq!(second, None);
    }

    #[test]
    fn test_wrap_message_breaks_on_word_boundary() {
        let text = "Remplacer \"X\" ? (o)ui / (n)on / (t)out / Echap pour arreter";
        let (first, second) = wrap_message_two_lines(text, 30);
        assert!(first.chars().count() <= 30);
        assert!(!first.ends_with(' '));
        let second = second.expect("le message devrait se couper sur 2 lignes");
        assert!(second.chars().count() <= 30);
        assert_eq!(first, "Remplacer \"X\" ? (o)ui / (n)on");
        assert_eq!(second, "/ (t)out / Echap pour arreter");
    }

    #[test]
    fn test_wrap_message_hard_breaks_long_word() {
        let text = "supercalifragilisticexpialidocious";
        let (first, second) = wrap_message_two_lines(text, 10);
        assert_eq!(first.chars().count(), 10);
        let second = second.expect("un mot plus long que la largeur doit etre coupe");
        assert_eq!(second.chars().count(), 10);
    }

    #[test]
    fn test_wrap_message_zero_width() {
        let (first, second) = wrap_message_two_lines("abc", 0);
        assert_eq!(first, "");
        assert_eq!(second, None);
    }

    #[test]
    fn test_custom_cursor_in_middle_of_text() {
        let mut ab = AppendBuffer::new();
        let written = append_with_custom_cursor(&mut ab, "hello", Some(2), 20);
        assert_eq!(written, 5);
        assert_eq!(ab.data(), "he\x1b[7ml\x1b[27mlo");
    }

    #[test]
    fn test_custom_cursor_at_end_with_room() {
        let mut ab = AppendBuffer::new();
        let written = append_with_custom_cursor(&mut ab, "hi", Some(2), 20);
        assert_eq!(written, 3);
        assert_eq!(ab.data(), "hi\x1b[7m \x1b[27m");
    }

    #[test]
    fn test_custom_cursor_at_end_when_line_is_full() {
        let mut ab = AppendBuffer::new();
        let written = append_with_custom_cursor(&mut ab, "hi", Some(2), 2);
        assert_eq!(written, 2);
        assert_eq!(ab.data(), "h\x1b[7mi\x1b[27m");
    }

    #[test]
    fn test_no_cursor_leaves_text_untouched() {
        let mut ab = AppendBuffer::new();
        let written = append_with_custom_cursor(&mut ab, "hello", None, 20);
        assert_eq!(written, 5);
        assert_eq!(ab.data(), "hello");
    }
}
