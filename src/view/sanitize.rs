/// Remplace les caracteres de controle (U+0000-U+001F, U+007F-U+009F) par `?`
/// pour qu'un texte venant de l'exterieur (nom de fichier, message de plugin...)
/// ne puisse pas etre interprete par le terminal comme une sequence d'echappement.
/// La tabulation est conservee.
pub fn neutralize_controls(text: &str) -> String {
    if !text.chars().any(is_dangerous) {
        return text.to_string();
    }
    text.chars()
        .map(|c| if is_dangerous(c) { '?' } else { c })
        .collect()
}

fn is_dangerous(c: char) -> bool {
    c != '\t' && c.is_control()
}

#[cfg(test)]
mod tests {
    use super::neutralize_controls;

    #[test]
    fn test_plain_text_is_unchanged() {
        assert_eq!(neutralize_controls("café — 日本語"), "café — 日本語");
    }

    #[test]
    fn test_escape_sequences_are_neutralized() {
        let cleaned = neutralize_controls("x\x1b]0;TITRE\x07.txt");
        assert_eq!(cleaned, "x?]0;TITRE?.txt");
        assert!(!cleaned.contains('\x1b'));
    }

    #[test]
    fn test_del_and_c1_controls_are_neutralized() {
        assert_eq!(neutralize_controls("a\u{7f}b\u{9b}c\u{85}d"), "a?b?c?d");
    }

    #[test]
    fn test_newline_and_carriage_return_are_neutralized() {
        assert_eq!(neutralize_controls("a\nb\rc"), "a?b?c");
    }

    #[test]
    fn test_tab_is_kept() {
        assert_eq!(neutralize_controls("a\tb"), "a\tb");
    }
}
