//! Systeme de traduction (i18n) de Cera.
//!
//! Chaque langue est un simple fichier JSON plat (cle -> texte traduit)
//! dans `src/i18n/lang/`. Ce format a ete choisi pour que contribuer une
//! traduction ou en corriger une ne demande aucune connaissance de Rust :
//! il suffit de dupliquer un des fichiers existants (par exemple
//! `lang/en.json`) et de traduire chaque valeur, en gardant les cles et
//! les `%s` (remplaces par une valeur au moment de l'affichage) intacts.
//!
//! Pour ajouter une langue :
//! 1. Copier `src/i18n/lang/en.json` vers `src/i18n/lang/<code>.json`
//!    (ex: `de.json` pour l'allemand) et traduire chaque valeur.
//! 2. Ajouter une entree dans `EMBEDDED_LANGUAGES` ci-dessous.
//!
//! Aucune autre modification du code n'est necessaire.

use crate::io::json;
use std::collections::HashMap;

/// Langue de repli utilisee si une cle est absente de la langue active,
/// ou si la langue demandee n'existe pas du tout.
const FALLBACK_LANG: &str = "en";

/// Langues embarquees dans le binaire au moment de la compilation. Chaque
/// fichier JSON correspondant doit exister sous `src/i18n/lang/<code>.json`.
const EMBEDDED_LANGUAGES: &[(&str, &str)] = &[
    ("fr", include_str!("lang/fr.json")),
    ("en", include_str!("lang/en.json")),
    ("es", include_str!("lang/es.json")),
];

pub struct Translations {
    lang: String,
    entries: HashMap<String, String>,
    fallback: HashMap<String, String>,
}

impl Translations {
    /// Traduit `key`. Si la cle est absente de la langue active, retombe
    /// sur l'anglais, puis sur la cle elle-meme (jamais de texte manquant
    /// a l'ecran, meme si une traduction est incomplete).
    pub fn t(&self, key: &str) -> String {
        self.entries
            .get(key)
            .or_else(|| self.fallback.get(key))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Comme `t`, mais remplace la premiere occurrence de `%s` dans le
    /// texte traduit par `value`.
    pub fn t1(&self, key: &str, value: &str) -> String {
        self.t(key).replacen("%s", value, 1)
    }

    pub fn lang(&self) -> &str {
        &self.lang
    }
}

fn parse_lang_file(text: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let result = json::parse(text);
    if let Some(value) = result.value {
        if let Some(obj) = value.as_object() {
            for (k, v) in obj.iter() {
                if let json::Value::String(s) = v {
                    map.insert(k.clone(), s.clone());
                }
            }
        }
    }
    map
}

fn embedded_json_for(lang: &str) -> Option<&'static str> {
    EMBEDDED_LANGUAGES
        .iter()
        .find(|(code, _)| *code == lang)
        .map(|(_, json)| *json)
}

/// Detecte la langue preferee du systeme a partir de `LANG`/`LC_ALL`
/// (format habituel `fr_FR.UTF-8`, `en_US`, `es`...), en ne gardant que le
/// code de langue a deux lettres avant `_` ou `.`. Retourne `None` si rien
/// n'est defini ou reconnu.
pub fn detect_system_language() -> Option<String> {
    for var in ["LC_ALL", "LANG"] {
        if let Ok(val) = std::env::var(var) {
            let code = val.split(['_', '.']).next().unwrap_or("").to_lowercase();
            if !code.is_empty() && code != "c" && code != "posix" {
                return Some(code);
            }
        }
    }
    None
}

/// Charge les traductions pour `requested_lang` (typiquement un reglage
/// explicite de `config.json`), ou detecte la langue du systeme si `None`.
/// Retombe sur l'anglais si la langue demandee/detectee n'est pas
/// disponible.
pub fn load(requested_lang: Option<&str>) -> Translations {
    let lang = requested_lang
        .map(|s| s.to_lowercase())
        .or_else(detect_system_language)
        .unwrap_or_else(|| FALLBACK_LANG.to_string());

    let resolved_lang = if embedded_json_for(&lang).is_some() {
        lang
    } else {
        FALLBACK_LANG.to_string()
    };

    let entries = embedded_json_for(&resolved_lang)
        .map(parse_lang_file)
        .unwrap_or_default();
    let fallback = embedded_json_for(FALLBACK_LANG)
        .map(parse_lang_file)
        .unwrap_or_default();

    Translations {
        lang: resolved_lang,
        entries,
        fallback,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_embedded_languages_parse_successfully() {
        for (code, text) in EMBEDDED_LANGUAGES {
            let map = parse_lang_file(text);
            assert!(
                !map.is_empty(),
                "le fichier de langue '{}' n'a pas pu etre parse ou est vide",
                code
            );
        }
    }

    #[test]
    fn test_all_languages_have_the_same_keys_as_fallback() {
        let fallback_json = embedded_json_for(FALLBACK_LANG).unwrap();
        let fallback_map = parse_lang_file(fallback_json);

        for (code, text) in EMBEDDED_LANGUAGES {
            if *code == FALLBACK_LANG {
                continue;
            }
            let map = parse_lang_file(text);
            for key in fallback_map.keys() {
                assert!(
                    map.contains_key(key),
                    "la langue '{}' n'a pas de traduction pour la cle '{}'",
                    code,
                    key
                );
            }
        }
    }

    #[test]
    fn test_load_explicit_language() {
        let tr = load(Some("es"));
        assert_eq!(tr.lang(), "es");
        assert_eq!(tr.t("help.save"), "guardar");
    }

    #[test]
    fn test_load_falls_back_to_english_for_unknown_language() {
        let tr = load(Some("xx"));
        assert_eq!(tr.lang(), "en");
    }

    #[test]
    fn test_missing_key_returns_key_itself() {
        let tr = load(Some("en"));
        assert_eq!(tr.t("this.key.does.not.exist"), "this.key.does.not.exist");
    }

    #[test]
    fn test_t1_replaces_placeholder() {
        let tr = load(Some("en"));
        assert_eq!(
            tr.t1("status.pattern_not_found", "foo"),
            "\"foo\" not found"
        );
    }

    #[test]
    fn test_detect_system_language_parses_locale_format() {
        std::env::set_var("LC_ALL", "fr_FR.UTF-8");
        assert_eq!(detect_system_language(), Some("fr".to_string()));
        std::env::remove_var("LC_ALL");
    }
}
