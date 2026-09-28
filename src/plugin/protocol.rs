//! Protocole de communication entre Cera et un plugin externe.
//!
//! Un plugin est un executable independant, dans n'importe quel langage.
//! Cera le lance en sous-processus et echange des messages JSON, un par
//! ligne, sur son entree/sortie standard :
//!
//! - Cera -> plugin : `init`, `key`, `resize`, `shutdown`.
//! - plugin -> Cera : `render` (affiche une vue plein ecran a la place du
//!   buffer), `open_file` (demande a Cera d'ouvrir un fichier en edition),
//!   `status` (message dans la barre de statut), `close` (ferme la vue et
//!   revient a l'edition).
//!
//! Ce fichier ne fait que construire/lire ces messages ; `manager.rs` gere
//! le sous-processus lui-meme.

use crate::io::json;

/// Echappe une chaine pour l'inserer dans une valeur JSON entre guillemets.
pub fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Un message Cera -> plugin, deja serialise en une ligne JSON (sans le
/// retour a la ligne final, ajoute par le manager au moment de l'ecriture).
pub struct OutgoingMessage(pub String);

impl OutgoingMessage {
    pub fn init(screen_rows: usize, screen_cols: usize, args: &[String]) -> Self {
        let args_json: Vec<String> = args
            .iter()
            .map(|a| format!("\"{}\"", json_escape(a)))
            .collect();
        Self(format!(
            "{{\"type\":\"init\",\"screen_rows\":{},\"screen_cols\":{},\"args\":[{}]}}",
            screen_rows,
            screen_cols,
            args_json.join(",")
        ))
    }

    pub fn key(key_name: &str) -> Self {
        Self(format!(
            "{{\"type\":\"key\",\"key\":\"{}\"}}",
            json_escape(key_name)
        ))
    }

    pub fn resize(screen_rows: usize, screen_cols: usize) -> Self {
        Self(format!(
            "{{\"type\":\"resize\",\"screen_rows\":{},\"screen_cols\":{}}}",
            screen_rows, screen_cols
        ))
    }

    pub fn shutdown() -> Self {
        Self("{\"type\":\"shutdown\"}".to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderSegment {
    pub text: String,
    /// Couleur hex ("#RRGGBB"), ou None pour heriter de la couleur normale
    /// du theme.
    pub color: Option<u32>,
    pub bold: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderLine {
    pub segments: Vec<RenderSegment>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IncomingMessage {
    Render {
        lines: Vec<RenderLine>,
    },
    OpenFile {
        path: String,
    },
    Status {
        message: String,
        is_error: bool,
    },
    Close,
    /// Message reconnu mais dont le contenu n'a pas pu etre interprete, ou
    /// type de message inconnu : ignore par l'appelant plutot que de
    /// planter, pour rester tolerant a un plugin partiellement bugue.
    Unknown,
}

fn parse_hex_color(s: &str) -> Option<u32> {
    let s = s.strip_prefix('#').unwrap_or(s);
    u32::from_str_radix(s, 16).ok()
}

/// Parse une ligne de sortie d'un plugin. Retourne `None` si la ligne
/// n'est pas un JSON valide (par exemple si le plugin a ecrit un message
/// de debug par erreur sur sa sortie standard) plutot que de faire
/// planter Cera.
pub fn parse_incoming(line: &str) -> Option<IncomingMessage> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let result = json::parse(trimmed);
    let value = result.value?;
    let msg_type = value.get("type")?.as_string("");

    match msg_type.as_str() {
        "render" => {
            let lines_val = value.get("lines")?.as_array()?;
            let mut lines = Vec::with_capacity(lines_val.len());
            for line_val in lines_val {
                let segments_val = line_val
                    .get("segments")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let mut segments = Vec::with_capacity(segments_val.len());
                for seg_val in &segments_val {
                    let text = seg_val
                        .get("text")
                        .map(|v| v.as_string(""))
                        .unwrap_or_default();
                    let color = seg_val
                        .get("color")
                        .filter(|v| v.is_string())
                        .and_then(|v| parse_hex_color(&v.as_string("")));
                    let bold = seg_val
                        .get("bold")
                        .map(|v| v.as_bool(false))
                        .unwrap_or(false);
                    segments.push(RenderSegment { text, color, bold });
                }
                lines.push(RenderLine { segments });
            }
            Some(IncomingMessage::Render { lines })
        }
        "open_file" => {
            let path = value.get("path")?.as_string("");
            if path.is_empty() {
                Some(IncomingMessage::Unknown)
            } else {
                Some(IncomingMessage::OpenFile { path })
            }
        }
        "status" => {
            let message = value
                .get("message")
                .map(|v| v.as_string(""))
                .unwrap_or_default();
            let kind = value
                .get("kind")
                .map(|v| v.as_string("info"))
                .unwrap_or_else(|| "info".to_string());
            Some(IncomingMessage::Status {
                message,
                is_error: kind == "error",
            })
        }
        "close" => Some(IncomingMessage::Close),
        _ => Some(IncomingMessage::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_escape_handles_quotes_and_backslashes() {
        assert_eq!(json_escape("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(json_escape("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_outgoing_init_message_shape() {
        let msg = OutgoingMessage::init(40, 120, &["/tmp/foo".to_string()]);
        assert_eq!(
            msg.0,
            "{\"type\":\"init\",\"screen_rows\":40,\"screen_cols\":120,\"args\":[\"/tmp/foo\"]}"
        );
    }

    #[test]
    fn test_outgoing_key_message_shape() {
        let msg = OutgoingMessage::key("Down");
        assert_eq!(msg.0, "{\"type\":\"key\",\"key\":\"Down\"}");
    }

    #[test]
    fn test_parse_render_message() {
        let line = r##"{"type":"render","lines":[{"segments":[{"text":"src/","color":"#82C0FF","bold":true}]}]}"##;
        let msg = parse_incoming(line).unwrap();
        match msg {
            IncomingMessage::Render { lines } => {
                assert_eq!(lines.len(), 1);
                assert_eq!(lines[0].segments.len(), 1);
                assert_eq!(lines[0].segments[0].text, "src/");
                assert_eq!(lines[0].segments[0].color, Some(0x82C0FF));
                assert!(lines[0].segments[0].bold);
            }
            _ => panic!("expected Render"),
        }
    }

    #[test]
    fn test_parse_open_file_message() {
        let line = r##"{"type":"open_file","path":"/tmp/foo.rs"}"##;
        assert_eq!(
            parse_incoming(line).unwrap(),
            IncomingMessage::OpenFile {
                path: "/tmp/foo.rs".to_string()
            }
        );
    }

    #[test]
    fn test_parse_status_message() {
        let line = r##"{"type":"status","message":"3 fichiers","kind":"info"}"##;
        assert_eq!(
            parse_incoming(line).unwrap(),
            IncomingMessage::Status {
                message: "3 fichiers".to_string(),
                is_error: false
            }
        );
    }

    #[test]
    fn test_parse_close_message() {
        let line = r##"{"type":"close"}"##;
        assert_eq!(parse_incoming(line).unwrap(), IncomingMessage::Close);
    }

    #[test]
    fn test_parse_invalid_json_returns_none() {
        assert_eq!(parse_incoming("not json at all"), None);
        assert_eq!(parse_incoming(""), None);
    }

    #[test]
    fn test_parse_unknown_type_is_unknown_not_error() {
        let line = r##"{"type":"something_else"}"##;
        assert_eq!(parse_incoming(line).unwrap(), IncomingMessage::Unknown);
    }

    #[test]
    fn test_parse_hex_color_with_and_without_hash() {
        assert_eq!(parse_hex_color("#82C0FF"), Some(0x82C0FF));
        assert_eq!(parse_hex_color("82C0FF"), Some(0x82C0FF));
        assert_eq!(parse_hex_color("zz"), None);
    }
}
