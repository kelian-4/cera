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
            normal: 0xE8EDF2,
            comment: 0x8FA3B8,
            keyword1: 0x82C0FF,
            keyword2: 0x8CF5D6,
            string_color: 0xA8E6CF,
            number: 0xC9A8FF,
            match_color: 0xFFFF66,
            line_number_fg: 0x7BA3C9,
            error_fg: 0xFF8A8A,
            success_fg: 0x9CE89C,
            selection_bg: 0x3A4A5A,
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub tab_stop: usize,
    pub quit_times: i32,
    pub true_color: bool,
    pub show_line_numbers: bool,
    pub mouse_enabled: bool,
    pub theme: ThemeColors,
    /// Code de langue force par l'utilisateur (ex: "fr", "en", "es"). Si
    /// absent, la langue du systeme (LANG/LC_ALL) est utilisee.
    pub language: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tab_stop: 8,
            quit_times: 1,
            true_color: true,
            show_line_numbers: true,
            mouse_enabled: true,
            theme: ThemeColors::default(),
            language: None,
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
    "config.json".to_string()
}

/// Le contenu JSON par defaut ecrit lors de la premiere generation du
/// fichier de configuration. Reflete exactement les valeurs par defaut de
/// `Config::default()` / `ThemeColors::default()`, formate lisiblement
/// pour qu'il soit facile a comprendre et modifier a la main.
fn default_config_json() -> String {
    r##"{
  "tab_stop": 8,
  "quit_times": 1,
  "true_color": true,
  "show_line_numbers": true,
  "mouse_enabled": true,
  "language": null,
  "theme": {
    "normal": "#E8EDF2",
    "comment": "#8FA3B8",
    "keyword1": "#82C0FF",
    "keyword2": "#8CF5D6",
    "string": "#A8E6CF",
    "number": "#C9A8FF",
    "match": "#FFFF66",
    "line_number_fg": "#7BA3C9",
    "error_fg": "#FF8A8A",
    "success_fg": "#9CE89C"
  }
}
"##
    .to_string()
}

/// Cree le fichier de configuration avec les valeurs par defaut s'il
/// n'existe pas deja (creant aussi le dossier parent si besoin). N'ecrase
/// jamais un fichier existant. Les erreurs d'ecriture sont ignorees
/// silencieusement : l'editeur doit pouvoir demarrer meme sans pouvoir
/// creer ce fichier (dossier en lecture seule, etc.), en utilisant les
/// valeurs par defaut en memoire.
pub fn ensure_config_file_exists(path: &str) {
    if fs::metadata(path).is_ok() {
        return;
    }
    if let Some(parent) = std::path::Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = fs::create_dir_all(parent);
        }
    }
    let _ = fs::write(path, default_config_json());
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
    if let Some(v) = root.get("language") {
        if v.is_string() {
            let s = v.as_string("");
            if !s.is_empty() {
                cfg.language = Some(s);
            }
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
        assert_eq!(cfg.quit_times, 1);
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

    #[test]
    fn test_default_config_path_ends_with_config_json() {
        let path = default_config_path();
        assert!(
            path.ends_with("config.json"),
            "le fichier de config doit s'appeler config.json, obtenu: {}",
            path
        );
    }

    #[test]
    fn test_ensure_config_file_exists_creates_file_with_parent_dir() {
        let dir = "/tmp/cera_rust_test_ensure_config_dir";
        let path = format!("{}/config.json", dir);
        let _ = fs::remove_dir_all(dir);

        assert!(fs::metadata(&path).is_err());
        ensure_config_file_exists(&path);
        assert!(fs::metadata(&path).is_ok());

        // Le fichier genere doit etre un JSON valide, rechargeable, et
        // correspondre aux valeurs par defaut.
        let (cfg, loaded) = load_from_file(&path);
        assert!(loaded);
        assert_eq!(cfg.tab_stop, 8);
        assert_eq!(cfg.quit_times, 1);
        assert!(cfg.true_color);

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_ensure_config_file_exists_never_overwrites_existing_file() {
        let path = "/tmp/cera_rust_test_ensure_no_overwrite.json";
        let mut f = fs::File::create(path).unwrap();
        writeln!(f, r##"{{"tab_stop": 2}}"##).unwrap();
        drop(f);

        ensure_config_file_exists(path);

        let (cfg, _) = load_from_file(path);
        assert_eq!(
            cfg.tab_stop, 2,
            "un fichier existant ne doit jamais etre ecrase"
        );

        let _ = fs::remove_file(path);
    }
}
