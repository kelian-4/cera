#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Success,
    Error,
}

pub struct HelpEntry {
    pub shortcut: String,
    pub description: String,
}

pub struct HelpCategory {
    pub name: String,
    pub entries: Vec<HelpEntry>,
}

pub struct ShortHelpEntry {
    pub key: String,
    pub label: String,
}

pub fn short_help_entries(i18n: &crate::i18n::Translations) -> Vec<ShortHelpEntry> {
    let entry = |key: &str, label_key: &str| ShortHelpEntry {
        key: key.to_string(),
        label: i18n.t(label_key),
    };
    vec![
        entry("^S", "help.save"),
        entry("^Q", "help.quit"),
        entry("^F", "help.find"),
        entry("^R", "help.replace"),
        entry("^Z", "help.undo"),
        entry("^Y", "help.redo"),
        entry("^X", "help.cut"),
        entry("^C", "help.copy"),
        entry("^V", "help.paste"),
        entry("^K", "help.cut_line"),
        entry("^A", "help.select_all"),
        entry("^L", "help.goto_line"),
        entry("^N", "help.next_file"),
        entry("^P", "help.prev_file"),
        entry("^D", "help.docs"),
    ]
}

pub fn full_documentation(
    i18n: &crate::i18n::Translations,
    plugins: &[crate::plugin::registry::PluginManifest],
) -> Vec<HelpCategory> {
    let entry = |shortcut: &str, desc_key: &str| HelpEntry {
        shortcut: shortcut.to_string(),
        description: i18n.t(desc_key),
    };
    let mut categories = vec![
        HelpCategory {
            name: i18n.t("doc.category.file"),
            entries: vec![entry("Ctrl-S", "doc.save"), entry("Ctrl-Q", "doc.quit")],
        },
        HelpCategory {
            name: i18n.t("doc.category.edit"),
            entries: vec![
                entry("Ctrl-Z", "doc.undo"),
                entry("Ctrl-Y", "doc.redo"),
                entry("Ctrl-X", "doc.cut"),
                entry("Ctrl-C", "doc.copy"),
                entry("Ctrl-V", "doc.paste"),
                entry("Ctrl-K", "doc.cut_line"),
                entry("Ctrl-A", "doc.select_all"),
            ],
        },
        HelpCategory {
            name: i18n.t("doc.category.search"),
            entries: vec![entry("Ctrl-F", "doc.find"), entry("Ctrl-R", "doc.replace")],
        },
        HelpCategory {
            name: i18n.t("doc.category.navigation"),
            entries: vec![
                entry("Ctrl-L", "doc.goto_line"),
                entry("Ctrl-N", "doc.next_file"),
                entry("Ctrl-P", "doc.prev_file"),
                HelpEntry {
                    shortcut: i18n.t("doc.arrows"),
                    description: i18n.t("doc.arrows_desc"),
                },
                HelpEntry {
                    shortcut: i18n.t("doc.ctrl_arrow"),
                    description: i18n.t("doc.ctrl_arrow_desc"),
                },
            ],
        },
        HelpCategory {
            name: i18n.t("doc.category.help"),
            entries: vec![
                entry("Ctrl-H", "doc.toggle_help"),
                entry("Ctrl-D", "doc.full_docs"),
                HelpEntry {
                    shortcut: "Echap".to_string(),
                    description: i18n.t("doc.escape"),
                },
            ],
        },
    ];
    if !plugins.is_empty() {
        categories.push(HelpCategory {
            name: i18n.t("doc.category.plugins"),
            entries: plugins
                .iter()
                .map(|plugin| HelpEntry {
                    shortcut: plugin.name.clone(),
                    description: plugin.doc_text().to_string(),
                })
                .collect(),
        });
    }
    categories
}

pub enum DocLine {
    Header(String),
    Entry {
        shortcut: String,
        description: String,
    },
    Blank,
}

pub fn flatten_documentation(categories: &[HelpCategory]) -> Vec<DocLine> {
    let mut lines = Vec::new();
    for category in categories {
        lines.push(DocLine::Header(category.name.clone()));
        for entry in &category.entries {
            lines.push(DocLine::Entry {
                shortcut: entry.shortcut.clone(),
                description: entry.description.clone(),
            });
        }
        lines.push(DocLine::Blank);
    }
    lines
}

pub fn doc_visible_rows(screenrows: usize) -> usize {
    screenrows.saturating_sub(2)
}

pub fn clamp_doc_scroll(scroll: usize, total_lines: usize, visible_rows: usize) -> usize {
    scroll.min(total_lines.saturating_sub(visible_rows))
}

pub fn cli_help_text(
    i18n: &crate::i18n::Translations,
    plugins: &[crate::plugin::registry::PluginManifest],
) -> String {
    let options = [
        ("-e, --explorer [dossier]", "cli.help.explorer"),
        ("-a, --afficher fichier...", "cli.help.show"),
        ("-f, --find nom", "cli.help.find"),
        ("-g, --grep motif [chemin...]", "cli.help.grep"),
        ("-d, --diff fichier1 fichier2...", "cli.help.diff"),
        ("-h, --help", "cli.help.help"),
        ("-v, --version", "cli.help.version"),
    ];
    let mut text = format!(
        "{}\n\n{}\n",
        i18n.t("cli.help.usage"),
        i18n.t("cli.help.options")
    );
    for (flags, description_key) in options {
        text.push_str(&format!("  {:<34} {}\n", flags, i18n.t(description_key)));
    }
    if !plugins.is_empty() {
        text.push_str(&format!("\n{}\n", i18n.t("cli.help.plugins")));
        for plugin in plugins {
            text.push_str(&format!("  {:<14} {}\n", plugin.name, plugin.doc_text()));
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::protocol::DisplayMode;
    use crate::plugin::registry::PluginManifest;

    fn manifest(name: &str, description: &str, help: &str) -> PluginManifest {
        PluginManifest {
            id: name.to_lowercase(),
            name: name.to_string(),
            description: description.to_string(),
            help: help.to_string(),
            command: "python3".to_string(),
            args: vec![],
            shortcut: None,
            mode: DisplayMode::Fullscreen,
        }
    }

    #[test]
    fn test_full_documentation_has_no_plugin_category_without_plugins() {
        let i18n = crate::i18n::load(Some("en"));
        let categories = full_documentation(&i18n, &[]);
        assert!(categories.iter().all(|c| c.name != "Plugins"));
    }

    #[test]
    fn test_full_documentation_lists_plugin_docs() {
        let i18n = crate::i18n::load(Some("en"));
        let plugins = vec![
            manifest("Finder", "desc", "cera -f name"),
            manifest("Other", "only description", ""),
        ];
        let categories = full_documentation(&i18n, &plugins);
        let category = categories.iter().find(|c| c.name == "Plugins").unwrap();
        assert_eq!(category.entries.len(), 2);
        assert_eq!(category.entries[0].shortcut, "Finder");
        assert_eq!(category.entries[0].description, "cera -f name");
        assert_eq!(category.entries[1].description, "only description");
    }

    #[test]
    fn test_cli_help_lists_options_and_plugin_docs() {
        let i18n = crate::i18n::load(Some("en"));
        let text = cli_help_text(&i18n, &[manifest("Finder", "desc", "cera -f name")]);
        assert!(text.contains("Usage: cera"));
        for flag in [
            "-e, --explorer",
            "-a, --afficher",
            "-f, --find",
            "-g, --grep",
            "-d, --diff",
            "-h, --help",
            "-v, --version",
        ] {
            assert!(text.contains(flag), "missing {flag}");
        }
        assert!(text.contains("Active plugins:"));
        assert!(text.contains("Finder"));
        assert!(text.contains("cera -f name"));
    }

    #[test]
    fn test_flatten_documentation_counts_header_entries_and_blank() {
        let categories = vec![
            HelpCategory {
                name: "A".to_string(),
                entries: vec![HelpEntry {
                    shortcut: "x".to_string(),
                    description: "d".to_string(),
                }],
            },
            HelpCategory {
                name: "B".to_string(),
                entries: vec![],
            },
        ];
        let lines = flatten_documentation(&categories);
        assert_eq!(lines.len(), (1 + 1 + 1) + (1 + 1));
        assert!(matches!(lines[0], DocLine::Header(_)));
        assert!(matches!(lines[1], DocLine::Entry { .. }));
        assert!(matches!(lines[2], DocLine::Blank));
    }

    #[test]
    fn test_clamp_doc_scroll_never_passes_the_last_page() {
        assert_eq!(clamp_doc_scroll(0, 30, 10), 0);
        assert_eq!(clamp_doc_scroll(15, 30, 10), 15);
        assert_eq!(clamp_doc_scroll(99, 30, 10), 20);
        assert_eq!(clamp_doc_scroll(5, 8, 10), 0);
    }

    #[test]
    fn test_doc_visible_rows_leaves_title_and_last_row() {
        assert_eq!(doc_visible_rows(24), 22);
        assert_eq!(doc_visible_rows(1), 0);
    }

    #[test]
    fn test_cli_help_without_plugins_has_no_plugin_section() {
        let i18n = crate::i18n::load(Some("en"));
        let text = cli_help_text(&i18n, &[]);
        assert!(!text.contains("Active plugins:"));
    }
}
