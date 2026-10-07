use std::io::{self, BufRead, Write};

const NAME: &str = "rust";

fn find_number(line: &str, key: &str) -> i64 {
    let pattern = format!("\"{}\":", key);
    let Some(at) = line.find(&pattern) else {
        return -1;
    };
    let digits: String = line[at + pattern.len()..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().unwrap_or(-1)
}

fn find_string(line: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", key);
    let start = line.find(&pattern)? + pattern.len();
    let length = line[start..].find('"')?;
    Some(line[start..start + length].to_string())
}

fn send(message: &str) {
    let mut out = io::stdout().lock();
    let _ = writeln!(out, "{}", message);
    let _ = out.flush();
}

fn render(text: &str) {
    send(&format!(
        "{{\"type\":\"render\",\"lines\":[{{\"segments\":[{{\"text\":\"{}\",\"color\":\"#82C0FF\",\"bold\":true}}]}}]}}",
        text
    ));
}

fn handle_key(key: &str) {
    match key {
        "q" => send("{\"type\":\"close\"}"),
        "o" => send("{\"type\":\"open_file\",\"path\":\"/tmp/cera-plugin-test.txt\"}"),
        "s" => send(&format!(
            "{{\"type\":\"status\",\"message\":\"status {}\",\"kind\":\"error\"}}",
            NAME
        )),
        other => render(&format!("key {} {}", NAME, other)),
    }
}

fn main() {
    send(&format!("debug: starting {}", NAME));
    for line in io::stdin().lock().lines().map_while(Result::ok) {
        let Some(kind) = find_string(&line, "type") else {
            continue;
        };
        match kind.as_str() {
            "init" => {
                let mode = find_string(&line, "mode").unwrap_or_else(|| "fullscreen".to_string());
                render(&format!(
                    "hello {} {} {}x{}",
                    NAME,
                    mode,
                    find_number(&line, "screen_rows"),
                    find_number(&line, "screen_cols")
                ));
            }
            "resize" => render(&format!(
                "resize {} {}x{}",
                NAME,
                find_number(&line, "screen_rows"),
                find_number(&line, "screen_cols")
            )),
            "key" => {
                if let Some(key) = find_string(&line, "key") {
                    handle_key(&key);
                }
            }
            "click" => render(&format!("click {} {}", NAME, find_number(&line, "row"))),
            "focus" => render(&format!(
                "focus {} {}",
                NAME,
                line.contains("\"focused\":true")
            )),
            "shutdown" => return,
            _ => {}
        }
    }
}
