#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MessageKind {
    Info,
    Success,
    Error,
}

pub struct HelpEntry {
    pub shortcut: &'static str,
    pub description: &'static str,
}

pub struct HelpCategory {
    pub name: &'static str,
    pub entries: &'static [HelpEntry],
}

pub fn short_help_line() -> &'static str {
    "^H aide  ^S sauver  ^Q quitter  ^F chercher  ^R remplacer  ^Z annuler  ^Y retablir  ^K couper ligne"
}

pub fn full_documentation() -> &'static [HelpCategory] {
    &[
        HelpCategory {
            name: "Fichier",
            entries: &[
                HelpEntry {
                    shortcut: "Ctrl-S",
                    description: "Sauvegarder le fichier courant",
                },
                HelpEntry {
                    shortcut: "Ctrl-Q",
                    description: "Quitter Cera (confirmation si modifications non sauvegardees)",
                },
            ],
        },
        HelpCategory {
            name: "Edition",
            entries: &[
                HelpEntry {
                    shortcut: "Ctrl-Z",
                    description: "Annuler la derniere action",
                },
                HelpEntry {
                    shortcut: "Ctrl-Y",
                    description: "Retablir l'action annulee",
                },
                HelpEntry {
                    shortcut: "Ctrl-X",
                    description: "Couper la selection vers le presse-papier",
                },
                HelpEntry {
                    shortcut: "Ctrl-C",
                    description: "Copier la selection vers le presse-papier",
                },
                HelpEntry {
                    shortcut: "Ctrl-V",
                    description: "Coller le contenu du presse-papier",
                },
                HelpEntry {
                    shortcut: "Ctrl-K",
                    description: "Couper la ligne entiere",
                },
            ],
        },
        HelpCategory {
            name: "Recherche",
            entries: &[
                HelpEntry {
                    shortcut: "Ctrl-F",
                    description: "Rechercher (Entree pour l'occurrence suivante, Echap pour arreter)",
                },
                HelpEntry {
                    shortcut: "Ctrl-R",
                    description: "Rechercher-remplacer, avec confirmation par occurrence",
                },
            ],
        },
        HelpCategory {
            name: "Navigation",
            entries: &[
                HelpEntry {
                    shortcut: "Ctrl-L",
                    description: "Aller a une ligne precise",
                },
                HelpEntry {
                    shortcut: "Ctrl-N",
                    description: "Passer au tampon suivant",
                },
                HelpEntry {
                    shortcut: "Ctrl-P",
                    description: "Passer au tampon precedent",
                },
                HelpEntry {
                    shortcut: "Fleches",
                    description: "Deplacer le curseur",
                },
                HelpEntry {
                    shortcut: "Ctrl-Fleche",
                    description: "Etendre la selection",
                },
            ],
        },
        HelpCategory {
            name: "Aide",
            entries: &[
                HelpEntry {
                    shortcut: "Ctrl-H",
                    description: "Afficher/masquer le bandeau d'aide",
                },
                HelpEntry {
                    shortcut: "Ctrl-D",
                    description: "Afficher cette documentation complete",
                },
                HelpEntry {
                    shortcut: "Echap",
                    description: "Revenir a l'edition du fichier",
                },
            ],
        },
    ]
}
