use crate::io::json;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub help: String,
    pub command: String,
    pub args: Vec<String>,
    pub shortcut: Option<String>,
    pub mode: crate::plugin::protocol::DisplayMode,
}

impl PluginManifest {
    pub fn doc_text(&self) -> &str {
        if self.help.is_empty() {
            &self.description
        } else {
            &self.help
        }
    }
}

fn default_plugins_directory() -> Option<PathBuf> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Some(PathBuf::from(xdg).join("cera").join("plugins"));
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return Some(
                PathBuf::from(home)
                    .join(".config")
                    .join("cera")
                    .join("plugins"),
            );
        }
    }
    None
}

pub fn plugins_directory(configured: Option<&str>) -> Option<PathBuf> {
    match configured {
        Some(dir) if !dir.is_empty() => Some(PathBuf::from(dir)),
        _ => default_plugins_directory(),
    }
}

fn parse_manifest(id: &str, plugin_dir: &Path, content: &str) -> Option<PluginManifest> {
    let result = json::parse(content);
    let value = result.value?;
    if !result.ok || !value.is_object() {
        return None;
    }

    let name = value
        .get("name")
        .map(|v| v.as_string(id))
        .unwrap_or_else(|| id.to_string());
    let description = value
        .get("description")
        .map(|v| v.as_string(""))
        .unwrap_or_default();
    let help = value
        .get("help")
        .filter(|v| v.is_string())
        .map(|v| v.as_string(""))
        .unwrap_or_default();
    let command_raw = value.get("command")?.as_string("");
    if command_raw.is_empty() {
        return None;
    }
    let command = if command_raw.contains('/') {
        plugin_dir.join(&command_raw).to_string_lossy().into_owned()
    } else {
        command_raw
    };

    let args = value
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter(|v| v.is_string())
                .map(|v| {
                    let s = v.as_string("");
                    if s.starts_with('/') {
                        s
                    } else {
                        plugin_dir.join(&s).to_string_lossy().into_owned()
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let shortcut = value
        .get("shortcut")
        .filter(|v| v.is_string())
        .map(|v| v.as_string(""))
        .filter(|s| !s.is_empty());

    let mode = match value
        .get("mode")
        .map(|v| v.as_string("fullscreen"))
        .as_deref()
    {
        Some("panel") => crate::plugin::protocol::DisplayMode::Panel,
        _ => crate::plugin::protocol::DisplayMode::Fullscreen,
    };

    Some(PluginManifest {
        id: id.to_string(),
        name,
        description,
        help,
        command,
        args,
        shortcut,
        mode,
    })
}

pub fn load_manifest(plugins_dir: &Path, id: &str) -> Option<PluginManifest> {
    let plugin_dir = plugins_dir.join(id);
    let manifest_path = plugin_dir.join("plugin.json");
    let content = std::fs::read_to_string(&manifest_path).ok()?;
    parse_manifest(id, &plugin_dir, &content)
}

pub fn load_enabled_plugins(plugins_dir: &Path, enabled: &[String]) -> Vec<PluginManifest> {
    enabled
        .iter()
        .filter_map(|id| load_manifest(plugins_dir, id))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn write_manifest(dir: &Path, id: &str, content: &str) {
        let plugin_dir = dir.join(id);
        fs::create_dir_all(&plugin_dir).unwrap();
        fs::write(plugin_dir.join("plugin.json"), content).unwrap();
    }

    #[test]
    fn test_load_manifest_basic_fields() {
        let dir = std::env::temp_dir().join("cera_test_plugins_basic");
        let _ = fs::remove_dir_all(&dir);
        write_manifest(
            &dir,
            "file-explorer",
            r##"{"name":"File Explorer","description":"Browse files","command":"python3","args":["main.py"],"shortcut":"^E"}"##,
        );

        let manifest = load_manifest(&dir, "file-explorer").expect("manifest should load");
        assert_eq!(manifest.id, "file-explorer");
        assert_eq!(manifest.name, "File Explorer");
        assert_eq!(manifest.description, "Browse files");
        assert_eq!(manifest.command, "python3");
        assert_eq!(manifest.shortcut, Some("^E".to_string()));
        assert_eq!(manifest.args.len(), 1);
        assert!(manifest.args[0].ends_with("main.py"));
        assert!(manifest.args[0].starts_with(dir.to_str().unwrap()));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_doc_text_prefers_help_then_falls_back_to_description() {
        let dir = std::env::temp_dir().join("cera_test_plugins_doc_text");
        let _ = fs::remove_dir_all(&dir);
        write_manifest(
            &dir,
            "with-help",
            r##"{"description":"Short description","help":"Usage text","command":"python3"}"##,
        );
        write_manifest(
            &dir,
            "without-help",
            r##"{"description":"Short description","command":"python3"}"##,
        );

        let with_help = load_manifest(&dir, "with-help").unwrap();
        assert_eq!(with_help.help, "Usage text");
        assert_eq!(with_help.doc_text(), "Usage text");

        let without_help = load_manifest(&dir, "without-help").unwrap();
        assert_eq!(without_help.help, "");
        assert_eq!(without_help.doc_text(), "Short description");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_manifest_missing_returns_none() {
        let dir = std::env::temp_dir().join("cera_test_plugins_missing");
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(load_manifest(&dir, "does-not-exist"), None);
    }

    #[test]
    fn test_load_manifest_without_command_returns_none() {
        let dir = std::env::temp_dir().join("cera_test_plugins_no_command");
        let _ = fs::remove_dir_all(&dir);
        write_manifest(&dir, "broken", r##"{"name":"Broken"}"##);
        assert_eq!(load_manifest(&dir, "broken"), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_enabled_plugins_skips_missing_ones() {
        let dir = std::env::temp_dir().join("cera_test_plugins_enabled");
        let _ = fs::remove_dir_all(&dir);
        write_manifest(
            &dir,
            "pretty-cat",
            r##"{"name":"Pretty Cat","command":"python3","args":["main.py"]}"##,
        );

        let enabled = vec!["pretty-cat".to_string(), "ghost-plugin".to_string()];
        let manifests = load_enabled_plugins(&dir, &enabled);
        assert_eq!(manifests.len(), 1);
        assert_eq!(manifests[0].id, "pretty-cat");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_command_with_slash_resolved_relative_to_plugin_dir() {
        let dir = std::env::temp_dir().join("cera_test_plugins_relcmd");
        let _ = fs::remove_dir_all(&dir);
        write_manifest(
            &dir,
            "custom",
            r##"{"name":"Custom","command":"./run.sh"}"##,
        );

        let manifest = load_manifest(&dir, "custom").expect("manifest should load");
        assert!(manifest.command.ends_with("run.sh"));
        assert!(manifest.command.contains("custom"));

        let _ = fs::remove_dir_all(&dir);
    }
}
