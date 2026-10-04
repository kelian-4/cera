use super::protocol::{parse_incoming, DisplayMode, IncomingMessage, OutgoingMessage};
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::process::{Child, Command, Stdio};

pub struct PluginProcess {
    child: Child,
    pending: String,
}

#[derive(Debug)]
pub enum SpawnError {
    Io(std::io::Error),
}

impl std::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl PluginProcess {
    #[allow(clippy::too_many_arguments)]
    pub fn spawn(
        command: &str,
        command_args: &[String],
        mode: DisplayMode,
        screen_rows: usize,
        screen_cols: usize,
        panel_width: Option<usize>,
        plugin_args: &[String],
    ) -> Result<Self, SpawnError> {
        let child = Command::new(command)
            .args(command_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(SpawnError::Io)?;

        if let Some(stdout) = &child.stdout {
            set_nonblocking(stdout.as_raw_fd());
        }

        let mut process = PluginProcess {
            child,
            pending: String::new(),
        };
        process.send(OutgoingMessage::init(
            mode,
            screen_rows,
            screen_cols,
            panel_width,
            plugin_args,
        ));
        Ok(process)
    }

    pub fn send(&mut self, message: OutgoingMessage) {
        if let Some(stdin) = self.child.stdin.as_mut() {
            let _ = stdin.write_all(message.0.as_bytes());
            let _ = stdin.write_all(b"\n");
            let _ = stdin.flush();
        }
    }

    pub fn poll_incoming(&mut self) -> Vec<IncomingMessage> {
        let mut messages = Vec::new();
        let Some(stdout) = self.child.stdout.as_mut() else {
            return messages;
        };

        let mut buf = [0u8; 4096];
        loop {
            match stdout.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    self.pending.push_str(&String::from_utf8_lossy(&buf[..n]));
                }
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::Interrupted =>
                {
                    break;
                }
                Err(_) => break,
            }
        }

        while let Some(pos) = self.pending.find('\n') {
            let line = self.pending[..pos].to_string();
            self.pending.drain(..=pos);
            if let Some(msg) = parse_incoming(&line) {
                messages.push(msg);
            }
        }

        messages
    }

    pub fn is_alive(&mut self) -> bool {
        matches!(self.child.try_wait(), Ok(None))
    }

    pub fn terminate(&mut self) {
        self.send(OutgoingMessage::shutdown());
        for _ in 0..20 {
            if !self.is_alive() {
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn set_nonblocking(fd: std::os::unix::io::RawFd) {
    if let Ok(flags) = fcntl(fd, FcntlArg::F_GETFL) {
        let mut flags = OFlag::from_bits_truncate(flags);
        flags.insert(OFlag::O_NONBLOCK);
        let _ = fcntl(fd, FcntlArg::F_SETFL(flags));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_roundtrip_with_cat() {
        let mut process =
            PluginProcess::spawn("cat", &[], DisplayMode::Fullscreen, 24, 80, None, &[])
                .expect("cat devrait exister");

        let mut received = Vec::new();
        for _ in 0..50 {
            received.extend(process.poll_incoming());
            if !received.is_empty() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }

        assert!(!received.is_empty(), "aucun message recu de cat");

        process.terminate();
        assert!(!process.is_alive());
    }

    #[test]
    fn test_spawn_nonexistent_command_fails() {
        let result = PluginProcess::spawn(
            "this-command-does-not-exist-cera",
            &[],
            DisplayMode::Fullscreen,
            24,
            80,
            None,
            &[],
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_poll_incoming_never_blocks_when_no_data() {
        let mut process =
            PluginProcess::spawn("cat", &[], DisplayMode::Fullscreen, 24, 80, None, &[])
                .expect("cat");
        let start = std::time::Instant::now();
        let _ = process.poll_incoming();
        assert!(start.elapsed() < std::time::Duration::from_millis(200));
        process.terminate();
    }
}

#[cfg(test)]
mod bundled_plugin_tests {
    use super::*;
    use crate::plugin::protocol::{DisplayMode, IncomingMessage, OutgoingMessage};
    use std::path::PathBuf;

    fn python_available() -> bool {
        Command::new("python3")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }

    fn plugin_script(name: &str) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("plugins");
        path.push(name);
        path.push("main.py");
        path.to_string_lossy().into_owned()
    }

    fn wait_for<F: FnMut(&[IncomingMessage]) -> bool>(
        process: &mut PluginProcess,
        mut done: F,
    ) -> Vec<IncomingMessage> {
        let mut all = Vec::new();
        for _ in 0..100 {
            all.extend(process.poll_incoming());
            if done(&all) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        all
    }

    #[test]
    fn test_pretty_cat_renders_file_with_line_numbers() {
        if !python_available() {
            return;
        }
        let file = std::env::temp_dir().join("cera_test_pretty_cat_input.py");
        std::fs::write(&file, "def hello():\n    return 1\n").unwrap();

        let mut process = PluginProcess::spawn(
            "python3",
            &[plugin_script("pretty-cat")],
            DisplayMode::Fullscreen,
            10,
            60,
            None,
            &[file.to_string_lossy().into_owned()],
        )
        .expect("python3 should start");

        let messages = wait_for(&mut process, |m| {
            m.iter()
                .any(|x| matches!(x, IncomingMessage::Render { .. }))
        });
        process.terminate();

        let render = messages
            .into_iter()
            .find_map(|m| match m {
                IncomingMessage::Render { lines, .. } => Some(lines),
                _ => None,
            })
            .expect("pretty-cat should send a render message");

        let all_text: String = render
            .iter()
            .flat_map(|l| l.segments.iter().map(|s| s.text.clone()))
            .collect();
        assert!(all_text.contains("def"));
        assert!(all_text.contains("hello"));
        assert!(render
            .iter()
            .flat_map(|l| l.segments.iter())
            .any(|s| s.text == "def" && s.color.is_some()));

        let _ = std::fs::remove_file(&file);
    }

    #[test]
    fn test_file_explorer_lists_directory_and_opens_file() {
        if !python_available() {
            return;
        }
        let dir = std::env::temp_dir().join("cera_test_explorer_dir");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("only.txt"), "x").unwrap();

        let mut process = PluginProcess::spawn(
            "python3",
            &[plugin_script("file-explorer")],
            DisplayMode::Panel,
            10,
            60,
            Some(28),
            &[dir.to_string_lossy().into_owned()],
        )
        .expect("python3 should start");

        let first = wait_for(&mut process, |m| {
            m.iter()
                .any(|x| matches!(x, IncomingMessage::Render { .. }))
        });
        let listing: String = first
            .iter()
            .filter_map(|m| match m {
                IncomingMessage::Render { lines, .. } => Some(lines),
                _ => None,
            })
            .flat_map(|lines| lines.iter())
            .flat_map(|l| l.segments.iter().map(|s| s.text.clone()))
            .collect();
        assert!(listing.contains("only.txt"));

        process.send(OutgoingMessage::key("Enter", true));
        let after = wait_for(&mut process, |m| {
            m.iter()
                .any(|x| matches!(x, IncomingMessage::OpenFile { .. }))
        });
        process.terminate();

        let opened = after.into_iter().find_map(|m| match m {
            IncomingMessage::OpenFile { path } => Some(path),
            _ => None,
        });
        let opened = opened.expect("file-explorer should request open_file");
        assert!(opened.ends_with("only.txt"));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
