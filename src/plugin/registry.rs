//! Decouverte des plugins installes sur disque.
//!
//! Un plugin est un dossier sous le repertoire de plugins (par defaut
//! `~/.config/cera/plugins`), contenant un manifeste `plugin.json` :
//!
//! ```json
//! {
//!   "name": "file-explorer",
//!   "description": "Explorateur de fichiers",
//!   "command": "python3",
//!   "args": ["main.py"],
//!   "shortcut": "^E"
//! }
//! ```
//!
//! `command` est l'executable a lancer (peut etre un chemin absolu ou un
//! nom cherche dans PATH, comme `python3`, `node`, `bash`...) ; `args` est
//! passe tel quel a la commande, sauf qu'un argument qui ne commence pas
//! par `/` est resolu par rapport au dossier du plugin (typiquement le
//! chemin du script principal, ex: "main.py" -> ".../plugins/mon-plugin/main.py"),
//! pour qu'un plugin puisse s'ecrire sans connaitre son propre chemin
//! d'installation. Un argument qui commence par `/` (chemin absolu) est
//! laisse tel quel.

use crate::io::json;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq)]
pub struct PluginManifest {
    /// Identifiant du plugin : le nom de son dossier, utilise dans
    /// `plugins.enabled` de config.json.
    pub id: String,
    pub name: String,
    pub description: String,
    pub command: String,
    pub args: Vec<String>,
    /// Raccourci suggere (informatif ; l'activation reelle du raccourci
    /// clavier est geree par l'editeur, pas par le plugin).
    pub shortcut: Option<String>,
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
    let command_raw = value.get("command")?.as_string("");
    if command_raw.is_empty() {
        return None;
    }
    // Une commande qui contient un separateur de chemin est resolue par
    // rapport au dossier du plugin (ex: "./main" ou "bin/tool"), pour
    // qu'un plugin n'ait pas besoin de connaitre son propre chemin
    // d'installation absolu. Un nom seul (ex: "python3", "node") est
    // cherche tel quel dans PATH.
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

    Some(PluginManifest {
        id: id.to_string(),
        name,
        description,
        command,
        args,
        shortcut,
    })
}

/// Charge le manifeste d'un plugin nomme `id`, cherche sous
/// `<plugins_dir>/<id>/plugin.json`. Retourne `None` si le dossier,
/// le manifeste, ou son contenu est absent/invalide -- un plugin
/// active dans la config mais introuvable sur le disque est
/// silencieusement ignore plutot que de faire planter Cera au
/// demarrage.
pub fn load_manifest(plugins_dir: &Path, id: &str) -> Option<PluginManifest> {
    let plugin_dir = plugins_dir.join(id);
    let manifest_path = plugin_dir.join("plugin.json");
    let content = std::fs::read_to_string(&manifest_path).ok()?;
    parse_manifest(id, &plugin_dir, &content)
}

/// Charge les manifestes de tous les plugins listes dans `enabled` et
/// presents sur le disque, dans le meme ordre que `enabled`. Un plugin
/// active mais introuvable ou dont le manifeste est invalide est
/// simplement absent du resultat.
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
