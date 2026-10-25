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
    pub type_color: u32,
    pub function_color: u32,
    pub constant: u32,
    pub operator: u32,
    pub preprocessor: u32,
    pub variable: u32,
    pub escape: u32,
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
            type_color: 0xFFD08A,
            function_color: 0xF9A8D4,
            constant: 0xFF8FA3,
            operator: 0xB4C2D4,
            preprocessor: 0xD4E27A,
            variable: 0xFFC6A5,
            escape: 0xFFB86B,
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
    pub language: Option<String>,
    pub plugins_enabled: Vec<String>,
    pub plugins_directory: Option<String>,
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
            plugins_enabled: vec![
                "file-explorer".to_string(),
                "pretty-cat".to_string(),
                "finder".to_string(),
                "diff".to_string(),
                "grep".to_string(),
            ],
            plugins_directory: None,
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

fn default_config_json() -> String {
    r##"{
  "tab_stop": 8,
  "quit_times": 1,
  "true_color": true,
  "show_line_numbers": true,
  "mouse_enabled": true,
  "language": null,
  "plugins": {
    "enabled": ["file-explorer", "pretty-cat", "finder", "diff", "grep"],
    "directory": null
  },
  "theme": {
    "normal": "#E8EDF2",
    "comment": "#8FA3B8",
    "keyword1": "#82C0FF",
    "keyword2": "#8CF5D6",
    "string": "#A8E6CF",
    "number": "#C9A8FF",
    "type": "#FFD08A",
    "function": "#F9A8D4",
    "constant": "#FF8FA3",
    "operator": "#B4C2D4",
    "preprocessor": "#D4E27A",
    "variable": "#FFC6A5",
    "escape": "#FFB86B",
    "match": "#FFFF66",
    "line_number_fg": "#7BA3C9",
    "error_fg": "#FF8A8A",
    "success_fg": "#9CE89C"
  }
}
"##
    .to_string()
}

pub fn ensure_config_file_exists(path: &str) {
    ensure_config_file_exists_with(path, crate::plugin::install::bundled_plugins_directory());
}

fn ensure_config_file_exists_with(path: &str, bundled_plugins: Option<std::path::PathBuf>) {
    let parent = std::path::Path::new(path)
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    let plugins_directory = parent.map(|parent| parent.join("plugins"));
    let version = env!("CARGO_PKG_VERSION");

    if fs::metadata(path).is_ok() {
        if let (Some(source), Some(destination)) = (&bundled_plugins, &plugins_directory) {
            crate::plugin::install::update_bundled_plugins(source, destination, version);
        }
        return;
    }
    if let Some(parent) = parent {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(path, default_config_json()).is_err() {
        return;
    }
    if let (Some(source), Some(destination)) = (&bundled_plugins, &plugins_directory) {
        crate::plugin::install::install_bundled_plugins(source, destination, version);
    }
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

    if let Some(plugins_obj) = root.get("plugins") {
        if let Some(v) = plugins_obj.get("enabled") {
            if let Some(arr) = v.as_array() {
                let names: Vec<String> = arr
                    .iter()
                    .filter(|item| item.is_string())
                    .map(|item| item.as_string(""))
                    .filter(|s| !s.is_empty())
                    .collect();
                cfg.plugins_enabled = names;
            }
        }
        if let Some(v) = plugins_obj.get("directory") {
            if v.is_string() {
                let s = v.as_string("");
                if !s.is_empty() {
                    cfg.plugins_directory = Some(s);
                }
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
            apply_theme_field(theme, "type", &mut cfg.theme.type_color);
            apply_theme_field(theme, "function", &mut cfg.theme.function_color);
            apply_theme_field(theme, "constant", &mut cfg.theme.constant);
            apply_theme_field(theme, "operator", &mut cfg.theme.operator);
            apply_theme_field(theme, "preprocessor", &mut cfg.theme.preprocessor);
            apply_theme_field(theme, "variable", &mut cfg.theme.variable);
            apply_theme_field(theme, "escape", &mut cfg.theme.escape);
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
        assert_eq!(
            cfg.plugins_enabled,
            vec![
                "file-explorer".to_string(),
                "pretty-cat".to_string(),
                "finder".to_string(),
                "diff".to_string(),
                "grep".to_string()
            ]
        );
        assert_eq!(cfg.plugins_directory, None);
    }

    #[test]
    fn test_config_reads_plugins_section() {
        let path = "/tmp/cera_rust_test_config_plugins.json";
        let mut f = fs::File::create(path).unwrap();
        writeln!(
            f,
            r##"{{"plugins": {{"enabled": ["file-explorer"], "directory": "/tmp/my-plugins"}}}}"##
        )
        .unwrap();

        let (cfg, loaded) = load_from_file(path);
        assert!(loaded);
        assert_eq!(cfg.plugins_enabled, vec!["file-explorer".to_string()]);
        assert_eq!(cfg.plugins_directory, Some("/tmp/my-plugins".to_string()));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_config_plugins_enabled_can_be_emptied() {
        let path = "/tmp/cera_rust_test_config_no_plugins.json";
        let mut f = fs::File::create(path).unwrap();
        writeln!(f, r##"{{"plugins": {{"enabled": []}}}}"##).unwrap();

        let (cfg, _) = load_from_file(path);
        assert!(cfg.plugins_enabled.is_empty());

        let _ = fs::remove_file(path);
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
    fn test_config_loads_syntax_class_colors() {
        let path = "/tmp/cera_rust_test_config_syntax_colors.json";
        let mut f = fs::File::create(path).unwrap();
        writeln!(
            f,
            r##"{{"theme": {{"type": "#010203", "function": "#040506", "constant": "#070809", "operator": "#0a0b0c", "preprocessor": "#0d0e0f", "variable": "#101112", "escape": "#131415"}}}}"##
        )
        .unwrap();

        let (cfg, loaded) = load_from_file(path);
        assert!(loaded);
        assert_eq!(cfg.theme.type_color, 0x010203);
        assert_eq!(cfg.theme.function_color, 0x040506);
        assert_eq!(cfg.theme.constant, 0x070809);
        assert_eq!(cfg.theme.operator, 0x0a0b0c);
        assert_eq!(cfg.theme.preprocessor, 0x0d0e0f);
        assert_eq!(cfg.theme.variable, 0x101112);
        assert_eq!(cfg.theme.escape, 0x131415);

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_default_config_json_lists_every_syntax_color() {
        let json = default_config_json();
        for key in [
            "type",
            "function",
            "constant",
            "operator",
            "preprocessor",
            "variable",
            "escape",
        ] {
            assert!(json.contains(&format!("\"{}\":", key)), "missing {key}");
        }
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

    fn bundled_plugins_fixture(name: &str) -> std::path::PathBuf {
        let source = std::path::PathBuf::from(format!("/tmp/cera_rust_test_bundled_{}", name));
        let _ = fs::remove_dir_all(&source);
        fs::create_dir_all(source.join("finder")).unwrap();
        fs::write(source.join("finder/plugin.json"), "{}").unwrap();
        source
    }

    #[test]
    fn test_ensure_config_installs_bundled_plugins_next_to_new_config() {
        let dir = "/tmp/cera_rust_test_ensure_installs";
        let _ = fs::remove_dir_all(dir);
        let source = bundled_plugins_fixture("installs");
        let path = format!("{}/config.json", dir);

        ensure_config_file_exists_with(&path, Some(source.clone()));

        assert!(fs::metadata(format!("{}/plugins/finder/plugin.json", dir)).is_ok());
        let _ = fs::remove_dir_all(dir);
        let _ = fs::remove_dir_all(source);
    }

    #[test]
    fn test_ensure_config_does_not_install_plugins_when_config_exists() {
        let dir = "/tmp/cera_rust_test_ensure_existing";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
        let path = format!("{}/config.json", dir);
        fs::write(&path, "{}").unwrap();
        let source = bundled_plugins_fixture("existing");

        ensure_config_file_exists_with(&path, Some(source.clone()));

        assert!(fs::metadata(format!("{}/plugins", dir)).is_err());
        let _ = fs::remove_dir_all(dir);
        let _ = fs::remove_dir_all(source);
    }

    #[test]
    fn test_ensure_config_without_bundled_plugins_creates_no_plugins_dir() {
        let dir = "/tmp/cera_rust_test_ensure_none";
        let _ = fs::remove_dir_all(dir);
        let path = format!("{}/config.json", dir);

        ensure_config_file_exists_with(&path, None);

        assert!(fs::metadata(&path).is_ok());
        assert!(fs::metadata(format!("{}/plugins", dir)).is_err());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn test_ensure_config_updates_installed_plugins_when_the_version_changes() {
        let dir = "/tmp/cera_rust_test_ensure_update";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(format!("{}/plugins/finder", dir)).unwrap();
        fs::write(format!("{}/plugins/finder/plugin.json", dir), "old").unwrap();
        fs::write(format!("{}/plugins/.cera-version", dir), "0.0.1").unwrap();
        let path = format!("{}/config.json", dir);
        fs::write(&path, "{}").unwrap();
        let source = bundled_plugins_fixture("update");

        ensure_config_file_exists_with(&path, Some(source.clone()));

        assert_eq!(
            fs::read_to_string(format!("{}/plugins/finder/plugin.json", dir)).unwrap(),
            "{}"
        );
        assert_eq!(
            fs::read_to_string(format!("{}/plugins/.cera-version", dir)).unwrap(),
            env!("CARGO_PKG_VERSION")
        );
        let _ = fs::remove_dir_all(dir);
        let _ = fs::remove_dir_all(source);
    }
}
