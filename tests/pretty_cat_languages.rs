use cera::plugin::manager::PluginProcess;
use cera::plugin::protocol::{DisplayMode, IncomingMessage, OutgoingMessage};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const KEYWORD1: (u8, u8, u8) = (0x82, 0xC0, 0xFF);
const KEYWORD2: (u8, u8, u8) = (0x8C, 0xF5, 0xD6);
const STRING: (u8, u8, u8) = (0xA8, 0xE6, 0xCF);
const NUMBER: (u8, u8, u8) = (0xC9, 0xA8, 0xFF);
const COMMENT: (u8, u8, u8) = (0x8F, 0xA3, 0xB8);
const TYPE: (u8, u8, u8) = (0xFF, 0xD0, 0x8A);
const FUNCTION: (u8, u8, u8) = (0xF9, 0xA8, 0xD4);
const CONSTANT: (u8, u8, u8) = (0xFF, 0x8F, 0xA3);
const PREPROCESSOR: (u8, u8, u8) = (0xD4, 0xE2, 0x7A);
const VARIABLE: (u8, u8, u8) = (0xFF, 0xC6, 0xA5);
const ESCAPE: (u8, u8, u8) = (0xFF, 0xB8, 0x6B);

struct Case {
    file: &'static str,
    content: &'static str,
    expect: &'static [(&'static str, (u8, u8, u8))],
}

const CASES: &[Case] = &[
    Case {
        file: "main.rs",
        content: "#[derive(Debug)]\nfn main() { let n: u8 = 7; /* c */ }\n",
        expect: &[
            ("#[derive", PREPROCESSOR),
            ("fn", KEYWORD1),
            ("main", FUNCTION),
            ("u8", TYPE),
            ("7", NUMBER),
            ("/* c */", COMMENT),
        ],
    },
    Case {
        file: "main.c",
        content: "#include <stdio.h>\nint main(void) { return 0; }\n",
        expect: &[
            ("#include", PREPROCESSOR),
            ("<stdio.h>", STRING),
            ("int", TYPE),
            ("main", FUNCTION),
            ("return", KEYWORD1),
            ("0", NUMBER),
        ],
    },
    Case {
        file: "main.cpp",
        content: "#include <vector>\nstd::vector<int> v = {1, 2};\n",
        expect: &[
            ("#include", PREPROCESSOR),
            ("vector", TYPE),
            ("int", TYPE),
            ("1", NUMBER),
        ],
    },
    Case {
        file: "main.py",
        content: "@app.route(\"/x\")\ndef calc(n: int = 0x1F):\n    return None  # c\n",
        expect: &[
            ("@app.route", PREPROCESSOR),
            ("def", KEYWORD1),
            ("calc", FUNCTION),
            ("int", TYPE),
            ("0x1F", NUMBER),
            ("None", CONSTANT),
            ("# c", COMMENT),
        ],
    },
    Case {
        file: "main.js",
        content: "const f = (x) => `a${x}`;\nclass Foo extends Bar {}\n",
        expect: &[
            ("const", KEYWORD2),
            ("`a${x}`", STRING),
            ("class", KEYWORD1),
            ("Foo", TYPE),
        ],
    },
    Case {
        file: "flake.nix",
        content: "{ pkgs }: let\n  x = 1; # c\nin pkgs.mkShell { y = \"a\\n\"; ok = true; }\n",
        expect: &[
            ("let", KEYWORD1),
            ("1", NUMBER),
            ("# c", COMMENT),
            ("in", KEYWORD1),
            ("mkShell", KEYWORD2),
            ("\\n", ESCAPE),
            ("true", CONSTANT),
        ],
    },
    Case {
        file: "run.sh",
        content: "echo $HOME | grep x # c\n",
        expect: &[("$HOME", VARIABLE), ("# c", COMMENT)],
    },
];

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn plugin_script() -> String {
    format!("{}/plugins/pretty-cat/main.py", env!("CARGO_MANIFEST_DIR"))
}

fn python_available() -> bool {
    Command::new("python3")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn require_python() -> bool {
    if python_available() {
        return true;
    }
    assert!(
        std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
        "python3 is required but not available"
    );
    eprintln!("SKIP: python3 not available");
    false
}

fn workdir(name: &str) -> TempDir {
    let dir = std::env::temp_dir().join(format!("cera_pretty_cat_{}_{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    TempDir(dir)
}

fn ansi(color: (u8, u8, u8)) -> String {
    format!("\x1b[38;2;{};{};{}m", color.0, color.1, color.2)
}

#[test]
fn cat_mode_colors_each_language_with_the_editor_palette() {
    if !require_python() {
        return;
    }
    let dir = workdir("cat");
    for case in CASES {
        let path = dir.0.join(case.file);
        std::fs::write(&path, case.content).unwrap();
        let output = Command::new("python3")
            .arg(plugin_script())
            .arg("--cat")
            .arg(&path)
            .output()
            .expect("pretty-cat should start");
        assert!(output.status.success(), "{} failed", case.file);
        let text = String::from_utf8_lossy(&output.stdout);
        for (token, color) in case.expect {
            let wanted = format!("{}{}\x1b[0m", ansi(*color), token);
            assert!(
                text.contains(&wanted),
                "{}: {:?} should be colored {:?}\n{:?}",
                case.file,
                token,
                color,
                text
            );
        }
    }
}

#[test]
fn cat_mode_keeps_unknown_file_types_uncolored() {
    if !require_python() {
        return;
    }
    let dir = workdir("unknown");
    let path = dir.0.join("data.xyz");
    std::fs::write(&path, "fn let 42\n").unwrap();
    let output = Command::new("python3")
        .arg(plugin_script())
        .arg("--cat")
        .arg(&path)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains(&format!("{}fn let 42\x1b[0m", ansi((0xE8, 0xED, 0xF2)))));
}

#[test]
fn cat_mode_reports_a_missing_file_and_continues() {
    if !require_python() {
        return;
    }
    let dir = workdir("missing");
    let good = dir.0.join("ok.py");
    std::fs::write(&good, "x = 1\n").unwrap();
    let output = Command::new("python3")
        .arg(plugin_script())
        .arg("--cat")
        .arg(dir.0.join("nope.py"))
        .arg(&good)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("nope.py"));
    assert!(String::from_utf8_lossy(&output.stdout).contains(&ansi(NUMBER)));
}

fn wait_render(plugin: &mut PluginProcess) -> Vec<cera::plugin::protocol::RenderLine> {
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(5) {
        for message in plugin.poll_incoming() {
            if let IncomingMessage::Render { lines, .. } = message {
                return lines;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("no render received");
}

#[test]
fn interactive_mode_sends_colored_segments_and_closes_on_q() {
    if !require_python() {
        return;
    }
    let dir = workdir("interactive");
    let path = dir.0.join("main.rs");
    std::fs::write(&path, "fn main() { let n: u8 = 7; }\n").unwrap();
    let mut plugin = PluginProcess::spawn(
        "python3",
        &[plugin_script()],
        DisplayMode::Fullscreen,
        24,
        80,
        None,
        &[path.to_string_lossy().into_owned()],
    )
    .expect("spawn");

    let lines = wait_render(&mut plugin);
    let first = &lines[1];
    let colored = |text: &str| {
        first
            .segments
            .iter()
            .find(|s| s.text == text)
            .map(|s| s.color)
    };
    assert_eq!(colored("fn"), Some(Some(0x82C0FF)));
    assert_eq!(colored("main"), Some(Some(0xF9A8D4)));
    assert_eq!(colored("u8"), Some(Some(0xFFD08A)));
    assert_eq!(colored("7"), Some(Some(0xC9A8FF)));

    plugin.send(OutgoingMessage::key("q", true));
    let started = Instant::now();
    let mut closed = false;
    while started.elapsed() < Duration::from_secs(5) && !closed {
        closed = plugin
            .poll_incoming()
            .iter()
            .any(|m| matches!(m, IncomingMessage::Close));
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(closed, "plugin should close on q");
    plugin.send(OutgoingMessage::shutdown());
}
