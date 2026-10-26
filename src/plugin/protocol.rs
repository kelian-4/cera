use crate::io::json;
use crate::view::sanitize::neutralize_controls;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayMode {
    Fullscreen,
    Panel,
}

impl DisplayMode {
    fn as_str(self) -> &'static str {
        match self {
            DisplayMode::Fullscreen => "fullscreen",
            DisplayMode::Panel => "panel",
        }
    }
}

pub struct OutgoingMessage(pub String);

impl OutgoingMessage {
    pub fn init(
        mode: DisplayMode,
        screen_rows: usize,
        screen_cols: usize,
        panel_width: Option<usize>,
        args: &[String],
    ) -> Self {
        let args_json: Vec<String> = args
            .iter()
            .map(|a| format!("\"{}\"", json_escape(a)))
            .collect();
        let panel_width_field = match mode {
            DisplayMode::Panel => format!(",\"panel_width\":{}", panel_width.unwrap_or(30)),
            DisplayMode::Fullscreen => String::new(),
        };
        Self(format!(
            "{{\"type\":\"init\",\"mode\":\"{}\",\"screen_rows\":{},\"screen_cols\":{}{},\"args\":[{}]}}",
            mode.as_str(),
            screen_rows,
            screen_cols,
            panel_width_field,
            args_json.join(",")
        ))
    }

    pub fn key(key_name: &str, focused: bool) -> Self {
        Self(format!(
            "{{\"type\":\"key\",\"key\":\"{}\",\"focused\":{}}}",
            json_escape(key_name),
            focused
        ))
    }

    pub fn click(row: usize) -> Self {
        Self(format!("{{\"type\":\"click\",\"row\":{}}}", row))
    }

    pub fn resize(screen_rows: usize, screen_cols: usize) -> Self {
        Self(format!(
            "{{\"type\":\"resize\",\"screen_rows\":{},\"screen_cols\":{}}}",
            screen_rows, screen_cols
        ))
    }

    pub fn focus(focused: bool) -> Self {
        Self(format!("{{\"type\":\"focus\",\"focused\":{}}}", focused))
    }

    pub fn shutdown() -> Self {
        Self("{\"type\":\"shutdown\"}".to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderSegment {
    pub text: String,
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
        panel_width: Option<usize>,
    },
    OpenFile {
        path: String,
    },
    Status {
        message: String,
        is_error: bool,
    },
    Close,
    Unknown,
}

fn parse_hex_color(s: &str) -> Option<u32> {
    let s = s.strip_prefix('#').unwrap_or(s);
    u32::from_str_radix(s, 16).ok()
}

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
                        .map(|v| neutralize_controls(&v.as_string("")))
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
            let panel_width = value
                .get("panel_width")
                .filter(|v| v.is_number())
                .map(|v| v.as_i64(0).max(0) as usize);
            Some(IncomingMessage::Render { lines, panel_width })
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
                .map(|v| neutralize_controls(&v.as_string("")))
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
    fn test_incoming_text_cannot_carry_terminal_escape_sequences() {
        let line =
            r##"{"type":"render","lines":[{"segments":[{"text":"x\u001b]0;TITRE\u0007.txt"}]}]}"##;
        let IncomingMessage::Render { lines, .. } = parse_incoming(line).unwrap() else {
            panic!("message render attendu");
        };
        assert_eq!(lines[0].segments[0].text, "x?]0;TITRE?.txt");

        let status = r##"{"type":"status","message":"a\u001b[2Jb","kind":"error"}"##;
        assert_eq!(
            parse_incoming(status).unwrap(),
            IncomingMessage::Status {
                message: "a?[2Jb".to_string(),
                is_error: true
            }
        );
    }

    #[test]
    fn test_open_file_path_is_not_altered() {
        let line = r##"{"type":"open_file","path":"/tmp/a\u001bb"}"##;
        assert_eq!(
            parse_incoming(line).unwrap(),
            IncomingMessage::OpenFile {
                path: "/tmp/a\u{1b}b".to_string()
            }
        );
    }

    #[test]
    fn test_json_escape_handles_quotes_and_backslashes() {
        assert_eq!(json_escape("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(json_escape("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_outgoing_init_message_shape_fullscreen() {
        let msg = OutgoingMessage::init(
            DisplayMode::Fullscreen,
            40,
            120,
            None,
            &["/tmp/foo".to_string()],
        );
        assert_eq!(
            msg.0,
            "{\"type\":\"init\",\"mode\":\"fullscreen\",\"screen_rows\":40,\"screen_cols\":120,\"args\":[\"/tmp/foo\"]}"
        );
    }

    #[test]
    fn test_outgoing_init_message_shape_panel() {
        let msg = OutgoingMessage::init(
            DisplayMode::Panel,
            40,
            120,
            Some(28),
            &["/tmp/foo".to_string()],
        );
        assert_eq!(
            msg.0,
            "{\"type\":\"init\",\"mode\":\"panel\",\"screen_rows\":40,\"screen_cols\":120,\"panel_width\":28,\"args\":[\"/tmp/foo\"]}"
        );
    }

    #[test]
    fn test_outgoing_key_message_shape() {
        let msg = OutgoingMessage::key("Down", true);
        assert_eq!(
            msg.0,
            "{\"type\":\"key\",\"key\":\"Down\",\"focused\":true}"
        );
    }

    #[test]
    fn test_outgoing_focus_message_shape() {
        let msg = OutgoingMessage::focus(false);
        assert_eq!(msg.0, "{\"type\":\"focus\",\"focused\":false}");
    }

    #[test]
    fn test_parse_render_message() {
        let line = r##"{"type":"render","lines":[{"segments":[{"text":"src/","color":"#82C0FF","bold":true}]}]}"##;
        let msg = parse_incoming(line).unwrap();
        match msg {
            IncomingMessage::Render { lines, panel_width } => {
                assert_eq!(lines.len(), 1);
                assert_eq!(lines[0].segments.len(), 1);
                assert_eq!(lines[0].segments[0].text, "src/");
                assert_eq!(lines[0].segments[0].color, Some(0x82C0FF));
                assert!(lines[0].segments[0].bold);
                assert_eq!(panel_width, None);
            }
            _ => panic!("expected Render"),
        }
    }

    #[test]
    fn test_parse_render_message_with_panel_width() {
        let line = r##"{"type":"render","lines":[],"panel_width":25}"##;
        let msg = parse_incoming(line).unwrap();
        match msg {
            IncomingMessage::Render { panel_width, .. } => {
                assert_eq!(panel_width, Some(25));
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
