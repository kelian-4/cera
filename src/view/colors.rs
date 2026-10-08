use crate::io::config::ThemeColors;
use crate::model::row::Highlight;

pub fn highlight_to_rgb(hl: Highlight, theme: &ThemeColors) -> u32 {
    match hl {
        Highlight::Comment | Highlight::MlComment => theme.comment,
        Highlight::Keyword1 => theme.keyword1,
        Highlight::Keyword2 => theme.keyword2,
        Highlight::String => theme.string_color,
        Highlight::Number => theme.number,
        Highlight::Type => theme.type_color,
        Highlight::Function => theme.function_color,
        Highlight::Constant => theme.constant,
        Highlight::Operator => theme.operator,
        Highlight::Preprocessor => theme.preprocessor,
        Highlight::Variable => theme.variable,
        Highlight::Escape => theme.escape,
        Highlight::Match => theme.match_color,
        Highlight::Normal => theme.normal,
    }
}

fn clamp6(v: i32) -> i32 {
    v.clamp(0, 5)
}

pub fn rgb_to_ansi256(rgb: u32) -> i32 {
    let r = ((rgb >> 16) & 0xFF) as i32;
    let g = ((rgb >> 8) & 0xFF) as i32;
    let b = (rgb & 0xFF) as i32;

    let r_idx = clamp6((r * 5 + 127) / 255);
    let g_idx = clamp6((g * 5 + 127) / 255);
    let b_idx = clamp6((b * 5 + 127) / 255);

    16 + 36 * r_idx + 6 * g_idx + b_idx
}

pub fn fg_sequence(rgb: u32, true_color: bool) -> String {
    if true_color {
        let r = (rgb >> 16) & 0xFF;
        let g = (rgb >> 8) & 0xFF;
        let b = rgb & 0xFF;
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    } else {
        format!("\x1b[38;5;{}m", rgb_to_ansi256(rgb))
    }
}

pub fn bg_sequence(rgb: u32, true_color: bool) -> String {
    if true_color {
        let r = (rgb >> 16) & 0xFF;
        let g = (rgb >> 8) & 0xFF;
        let b = rgb & 0xFF;
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    } else {
        format!("\x1b[48;5;{}m", rgb_to_ansi256(rgb))
    }
}

pub fn reset_fg_sequence() -> &'static str {
    "\x1b[39m"
}

pub fn reset_bg_sequence() -> &'static str {
    "\x1b[49m"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_true_color_sequence() {
        let s = fg_sequence(0xFF0000, true);
        assert_eq!(s, "\x1b[38;2;255;0;0m");
    }

    #[test]
    fn test_ansi256_white() {
        let idx = rgb_to_ansi256(0xFFFFFF);
        assert_eq!(idx, 231);
    }
}
