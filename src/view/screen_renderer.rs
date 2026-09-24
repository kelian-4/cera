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
    pub prompt_cursor_offset: Option<usize>,
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

/// Decoupe `text` en au plus 2 lignes d'au plus `width` caracteres chacune,
/// en coupant sur une limite de mot (espace) quand c'est possible. Si un
/// seul mot depasse `width`, il est coupe durement. Le texte au-dela de ce
/// que 2 lignes de `width` peuvent contenir est tronque.
pub fn wrap_message_two_lines(text: &str, width: usize) -> (String, Option<String>) {
    if width == 0 {
        return (String::new(), None);
    }
    let char_count = text.chars().count();
    if char_count <= width {
        return (text.to_string(), None);
    }

    let chars: Vec<char> = text.chars().collect();

    // Cherche la derniere limite de mot (espace) a l'interieur de `width`.
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

pub fn text_area_screen_rows(state: &ViewState) -> usize {
    let mut reserved = status_bar_rows(state) + message_bar_rows(state);
    if state.buffer_manager.count() > 1 {
        reserved += 1;
    }
    if state.help_active {
        reserved += 2;
    }
    state.screenrows.saturating_sub(reserved).max(1)
}

/// Nombre de lignes que prendra la barre de statut (prompt ou barre normale).
/// La barre de statut normale tient toujours sur 1 ligne (contenu structure,
/// jamais coupe). Seul un `prompt_text` long peut necessiter 2 lignes.
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

/// Nombre de lignes que prendra la barre de message (succes/erreur), 0 si
/// aucun message n'est actuellement affiche.
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
            let is_single_empty_line = buffer.num_rows() == 1 && buffer.row(0).chars().is_empty();
            if is_single_empty_line && y == text_rows / 3 {
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
    let text_width = min_column_width.saturating_sub(1).max(1);
    let max_entries = columns_per_line * 2;

    // Chaque colonne empile 2 entrees (ligne du haut, ligne du bas). On
    // calcule une largeur de colonne commune aux deux lignes (le plus long
    // des deux textes) pour garantir un alignement vertical exact, plutot
    // que de justifier chaque ligne independamment.
    let mut columns: Vec<(String, Option<String>)> = Vec::new();
    for pair in entries
        .iter()
        .take(max_entries)
        .collect::<Vec<_>>()
        .chunks(2)
    {
        let top = format_help_cell(pair[0], text_width);
        let bottom = pair.get(1).map(|e| format_help_cell(e, text_width));
        columns.push((top, bottom));
    }

    let top_cells: Vec<String> = columns.iter().map(|(t, _)| t.clone()).collect();
    let bottom_cells: Vec<String> = columns
        .iter()
        .map(|(_, b)| b.clone().unwrap_or_default())
        .collect();
    let column_widths: Vec<usize> = columns
        .iter()
        .map(|(t, b)| {
            let bl = b.as_ref().map(|s| s.chars().count()).unwrap_or(0);
            t.chars().count().max(bl)
        })
        .collect();

    draw_help_grid_line(ab, state.screencols, &top_cells, &column_widths);
    draw_help_grid_line(ab, state.screencols, &bottom_cells, &column_widths);
}

fn format_help_cell(entry: &crate::view::help_text::ShortHelpEntry, text_width: usize) -> String {
    let cell = format!("{} {}", entry.key, entry.label);
    let cell_len = cell.chars().count();
    if cell_len > text_width {
        cell.chars().take(text_width).collect()
    } else {
        cell
    }
}

/// Dessine une ligne du bandeau d'aide avec des colonnes de largeur fixe
/// (`column_widths`), identiques sur les deux lignes du bandeau, pour que
/// chaque colonne s'aligne verticalement lettre pour lettre. L'espace
/// restant est reparti en marges egales entre les colonnes (bords colles
/// aux marges, comme le format justifie existant).
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

        // Complete jusqu'a la largeur de la colonne pour que la colonne
        // suivante demarre au meme endroit sur les deux lignes.
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

/// Dessine la barre de statut. Retourne, si un curseur de saisie doit etre
/// affiche, sa position relative (ligne 0 ou 1 dans la zone dessinee ici,
/// colonne 0-indexee) pour que `refresh` puisse y placer le curseur reel.
fn draw_status_bar(ab: &mut AppendBuffer, state: &ViewState) -> Option<(usize, usize)> {
    if let Some(prompt_text) = &state.prompt_text {
        let line = format!(" {}", prompt_text);
        let (first, second) = wrap_message_two_lines(&line, state.screencols);

        let first_len = first.chars().count();
        ab.append(&first);
        for _ in first_len..state.screencols {
            ab.append(" ");
        }
        ab.append("\r\n");

        let mut cursor_pos = None;
        // Le curseur logique se trouve a `1 + offset` dans `line` (le " "
        // ajoute en prefixe compte pour 1). On localise sur quelle ligne
        // affichee (avant/apres le wrap) il tombe.
        if let Some(offset) = state.prompt_cursor_offset {
            let target = (1 + offset).min(line.chars().count());
            if target <= first_len {
                cursor_pos = Some((0, target));
            } else if second.is_some() {
                cursor_pos = Some((1, target - first_len));
            } else {
                cursor_pos = Some((0, first_len));
            }
        }

        if let Some(second) = second {
            let second_len = second.chars().count();
            ab.append(&second);
            for _ in second_len..state.screencols {
                ab.append(" ");
            }
            ab.append("\r\n");
        }
        return cursor_pos;
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

    let search_cursor_pos = if state.search.active {
        let prefix_len = " recherche: ".chars().count();
        let target = (prefix_len + state.search_term.chars().count()).min(left_len);
        Some((0, target))
    } else {
        None
    };

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

    search_cursor_pos
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
    let status_cursor = draw_status_bar(&mut ab, state);
    draw_help_bar(&mut ab, state);
    draw_message_bar(&mut ab, state);

    let ob = state.buffer_manager.active();
    let gutter = gutter_width(state);
    let tab_bar_height = if state.buffer_manager.count() > 1 {
        1
    } else {
        0
    };

    if let Some((rel_row, col)) = status_cursor {
        let status_bar_row = tab_bar_height + text_area_screen_rows(state);
        let cursor_row = status_bar_row + rel_row + 1;
        let cursor_col = col + 1;
        ab.append(&format!("\x1b[{};{}H", cursor_row, cursor_col));
        ab.append("\x1b[?25h");
        return ab.data().to_string();
    }

    let cursor_row = tab_bar_height + (ob.cursor.cy.saturating_sub(state.rowoff)) + 1;
    let cursor_col = gutter + (ob.cursor.rx.saturating_sub(state.coloff)) + 1;
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
        // Aucune coupure au milieu d'un mot : la premiere ligne se termine
        // par un mot complet, la deuxieme reprend au mot suivant.
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
}
