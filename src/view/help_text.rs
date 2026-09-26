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
        entry("^L", "help.goto_line"),
        entry("^N", "help.next_file"),
        entry("^P", "help.prev_file"),
        entry("^D", "help.docs"),
    ]
}

pub fn full_documentation(i18n: &crate::i18n::Translations) -> Vec<HelpCategory> {
    let entry = |shortcut: &str, desc_key: &str| HelpEntry {
        shortcut: shortcut.to_string(),
        description: i18n.t(desc_key),
    };
    vec![
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
    ]
}
