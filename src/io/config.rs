use std::fs;

use super::json;

#[derive(Clone, Copy)]
pub struct ThemeColors {
    pub normal: u32,
    pub comment: u32,
    pub keyword1: u32,
    pub keyword2: u32,
    pub string_color: u32,
    pub number: u32,
    pub match_color: u32,
    pub line_number_fg: u32,
    pub error_fg: u32,
    pub success_fg: u32,
    pub selection_bg: u32,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            normal: 0xD8DEE9,
            comment: 0x616E88,
            keyword1: 0x81A1C1,
            keyword2: 0xA3BE8C,
            string_color: 0xEBCB8B,
            number: 0xB48EAD,
            match_color: 0xFFFF00,
            line_number_fg: 0x4C566A,
            error_fg: 0xBF616A,
            success_fg: 0xA3BE8C,
            selection_bg: 0x434C5E,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    pub tab_stop: usize,
    pub quit_times: i32,
    pub true_color: bool,
    pub show_line_numbers: bool,
    pub mouse_enabled: bool,
    pub theme: ThemeColors,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tab_stop: 8,
            quit_times: 3,
            true_color: true,
            show_line_numbers: true,
            mouse_enabled: true,
            theme: ThemeColors::default(),
        }
    }
}

fn parse_hex_color(s: &str, fallback: u32) -> u32 {
    let hex = s.strip_prefix('#').unwrap_or(s);
    if hex.len() != 6 {
        return fallback;
    }
    u32::from_str_radix(hex, 16).unwrap_or(fallback)
}

fn apply_theme_field(theme_obj: &json::Value, key: &str, target: &mut u32) {
    if let Some(v) = theme_obj.get(key) {
        if v.is_string() {
            *target = parse_hex_color(&v.as_string(""), *target);
        }
    }
}

pub fn default_config_path() -> String {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return format!("{}/cera/config.json", xdg);
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return format!("{}/.config/cera/config.json", home);
        }
    }
    ".cera.json".to_string()
}

pub fn load_from_file(path: &str) -> (Config, bool) {
    let mut cfg = Config::default();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (cfg, false),
    };

    let result = json::parse(&content);
    let root = match result.value {
        Some(v) if v.is_object() && result.ok => v,
        _ => return (cfg, false),
    };

    if let Some(v) = root.get("tab_stop") {
        if v.is_number() {
            cfg.tab_stop = v.as_i64(cfg.tab_stop as i64).max(1) as usize;
        }
    }
    if let Some(v) = root.get("quit_times") {
        if v.is_number() {
            cfg.quit_times = v.as_i64(cfg.quit_times as i64) as i32;
        }
    }
    if let Some(v) = root.get("true_color") {
        if v.is_bool() {
            cfg.true_color = v.as_bool(cfg.true_color);
        }
    }
    if let Some(v) = root.get("show_line_numbers") {
        if v.is_bool() {
            cfg.show_line_numbers = v.as_bool(cfg.show_line_numbers);
        }
    }
    if let Some(v) = root.get("mouse_enabled") {
        if v.is_bool() {
            cfg.mouse_enabled = v.as_bool(cfg.mouse_enabled);
        }
    }

    if let Some(theme) = root.get("theme") {
        if theme.is_object() {
            apply_theme_field(theme, "normal", &mut cfg.theme.normal);
            apply_theme_field(theme, "comment", &mut cfg.theme.comment);
            apply_theme_field(theme, "keyword1", &mut cfg.theme.keyword1);
            apply_theme_field(theme, "keyword2", &mut cfg.theme.keyword2);
            apply_theme_field(theme, "string", &mut cfg.theme.string_color);
            apply_theme_field(theme, "number", &mut cfg.theme.number);
            apply_theme_field(theme, "match", &mut cfg.theme.match_color);
            apply_theme_field(theme, "line_number_fg", &mut cfg.theme.line_number_fg);
            apply_theme_field(theme, "error_fg", &mut cfg.theme.error_fg);
            apply_theme_field(theme, "success_fg", &mut cfg.theme.success_fg);
        }
    }

    (cfg, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_defaults_on_missing_file() {
        let (cfg, loaded) = load_from_file("/nonexistent/path/config.json");
        assert!(!loaded);
        assert_eq!(cfg.tab_stop, 8);
        assert_eq!(cfg.quit_times, 3);
    }

    #[test]
    fn test_config_loads_real_file() {
        let path = "/tmp/cera_rust_test_config.json";
        let mut f = fs::File::create(path).unwrap();
        writeln!(
            f,
            r##"{{"tab_stop": 4, "quit_times": 1, "true_color": false, "theme": {{"keyword1": "#ff0000"}}}}"##
        )
        .unwrap();

        let (cfg, loaded) = load_from_file(path);
        assert!(loaded);
        assert_eq!(cfg.tab_stop, 4);
        assert_eq!(cfg.quit_times, 1);
        assert!(!cfg.true_color);
        assert_eq!(cfg.theme.keyword1, 0xff0000);

        let _ = fs::remove_file(path);
    }
}
