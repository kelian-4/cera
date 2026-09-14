use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub struct Clipboard {
    system_tool_available: bool,
    tool_name: String,
    copy_program: String,
    copy_args: Vec<String>,
    paste_program: String,
    paste_args: Vec<String>,
    fallback: String,
}

fn command_exists(name: &str) -> bool {
    Command::new("sh")
        .arg("-c")
        .arg(format!("command -v {} > /dev/null 2>&1", name))
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

impl Clipboard {
    pub fn new() -> Self {
        let mut clip = Clipboard {
            system_tool_available: false,
            tool_name: String::new(),
            copy_program: String::new(),
            copy_args: Vec::new(),
            paste_program: String::new(),
            paste_args: Vec::new(),
            fallback: String::new(),
        };
        clip.detect_tool();
        clip
    }

    fn detect_tool(&mut self) {
        let wayland_display = std::env::var("WAYLAND_DISPLAY").is_ok();
        if wayland_display && command_exists("wl-copy") && command_exists("wl-paste") {
            self.tool_name = "wl-clipboard".to_string();
            self.copy_program = "wl-copy".to_string();
            self.paste_program = "wl-paste".to_string();
            self.paste_args = vec!["--no-newline".to_string()];
            self.system_tool_available = true;
            return;
        }
        if command_exists("xclip") {
            self.tool_name = "xclip".to_string();
            self.copy_program = "xclip".to_string();
            self.copy_args = vec!["-selection".to_string(), "clipboard".to_string(), "-in".to_string()];
            self.paste_program = "xclip".to_string();
            self.paste_args = vec!["-selection".to_string(), "clipboard".to_string(), "-out".to_string()];
            self.system_tool_available = true;
            return;
        }
        if command_exists("xsel") {
            self.tool_name = "xsel".to_string();
            self.copy_program = "xsel".to_string();
            self.copy_args = vec!["--clipboard".to_string(), "--input".to_string()];
            self.paste_program = "xsel".to_string();
            self.paste_args = vec!["--clipboard".to_string(), "--output".to_string()];
            self.system_tool_available = true;
            return;
        }
        if command_exists("pbcopy") && command_exists("pbpaste") {
            self.tool_name = "pbcopy/pbpaste".to_string();
            self.copy_program = "pbcopy".to_string();
            self.paste_program = "pbpaste".to_string();
            self.system_tool_available = true;
            return;
        }

        self.tool_name = "interne (aucun outil systeme trouve)".to_string();
        self.system_tool_available = false;
    }

    pub fn using_system_clipboard(&self) -> bool {
        self.system_tool_available
    }

    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    pub fn copy(&mut self, text: &str) -> bool {
        self.fallback = text.to_string();

        if !self.system_tool_available {
            return false;
        }

        let child = Command::new(&self.copy_program)
            .args(&self.copy_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();

        let mut child = match child {
            Ok(c) => c,
            Err(_) => {
                self.system_tool_available = false;
                return false;
            }
        };

        let write_ok = if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).is_ok()
        } else {
            false
        };

        let _ = child.wait();

        if !write_ok {
            self.system_tool_available = false;
        }
        write_ok
    }

    pub fn paste(&mut self) -> String {
        if !self.system_tool_available {
            return self.fallback.clone();
        }

        let output = Command::new(&self.paste_program)
            .args(&self.paste_args)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output();

        match output {
            Ok(out) => {
                let mut result = String::new();
                let mut bytes = out.stdout.as_slice();
                let _ = bytes.read_to_string(&mut result);
                if result.is_empty() && !self.fallback.is_empty() {
                    self.fallback.clone()
                } else {
                    result
                }
            }
            Err(_) => {
                self.system_tool_available = false;
                self.fallback.clone()
            }
        }
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_fallback_roundtrip() {
        let mut clip = Clipboard::new();
        clip.copy("test clipboard content");
        assert_eq!(clip.paste(), "test clipboard content");
    }
}
