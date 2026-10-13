use cera::plugin::manager::PluginProcess;
use cera::plugin::protocol::{DisplayMode, IncomingMessage, OutgoingMessage};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const WAIT_LIMIT: Duration = Duration::from_secs(5);

struct PluginLanguage {
    name: &'static str,
    source: &'static str,
    tool: &'static str,
    compile_flags: Option<&'static [&'static str]>,
}

const PYTHON: PluginLanguage = PluginLanguage {
    name: "python",
    source: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/plugin_examples/hello.py"
    ),
    tool: "python3",
    compile_flags: None,
};

const JAVASCRIPT: PluginLanguage = PluginLanguage {
    name: "js",
    source: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/plugin_examples/hello.js"
    ),
    tool: "node",
    compile_flags: None,
};

const C: PluginLanguage = PluginLanguage {
    name: "c",
    source: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/plugin_examples/hello.c"),
    tool: "cc",
    compile_flags: Some(&["-O1"]),
};

const CPP: PluginLanguage = PluginLanguage {
    name: "cpp",
    source: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/plugin_examples/hello.cpp"
    ),
    tool: "c++",
    compile_flags: Some(&["-O1", "-std=c++17"]),
};

const RUST: PluginLanguage = PluginLanguage {
    name: "rust",
    source: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/plugin_examples/hello.rs"
    ),
    tool: "rustc",
    compile_flags: Some(&["--edition", "2021", "-O"]),
};

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn tool_available(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn prepare(language: &PluginLanguage, dir: &Path) -> (String, Vec<String>) {
    let Some(flags) = language.compile_flags else {
        return (language.tool.to_string(), vec![language.source.to_string()]);
    };
    let binary = dir.join("plugin");
    let status = Command::new(language.tool)
        .args(flags)
        .arg("-o")
        .arg(&binary)
        .arg(language.source)
        .status()
        .expect("compiler should start");
    assert!(
        status.success(),
        "{} plugin failed to compile",
        language.name
    );
    (binary.to_string_lossy().into_owned(), Vec::new())
}

fn wait_for(
    plugin: &mut PluginProcess,
    description: &str,
    accept: impl Fn(&IncomingMessage) -> bool,
) -> IncomingMessage {
    let started = Instant::now();
    while started.elapsed() < WAIT_LIMIT {
        for message in plugin.poll_incoming() {
            if accept(&message) {
                return message;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("timed out waiting for: {}", description);
}

fn first_segment_text(message: &IncomingMessage) -> Option<&str> {
    match message {
        IncomingMessage::Render { lines, .. } => lines
            .first()
            .and_then(|line| line.segments.first())
            .map(|segment| segment.text.as_str()),
        _ => None,
    }
}

fn expect_render(plugin: &mut PluginProcess, expected: &str) {
    let message = wait_for(plugin, expected, |m| {
        first_segment_text(m) == Some(expected)
    });
    let IncomingMessage::Render { lines, .. } = message else {
        unreachable!();
    };
    let segment = &lines[0].segments[0];
    assert_eq!(segment.color, Some(0x82C0FF), "color of '{}'", expected);
    assert!(segment.bold, "bold of '{}'", expected);
}

fn exercise(name: &str, command: &str, args: &[String], mode: DisplayMode) {
    let mode_name = match mode {
        DisplayMode::Fullscreen => "fullscreen",
        DisplayMode::Panel => "panel",
    };
    let panel_width = (mode == DisplayMode::Panel).then_some(28);
    let mut plugin = PluginProcess::spawn(
        command,
        args,
        mode,
        24,
        80,
        panel_width,
        &["/tmp/cera-test-file".to_string()],
    )
    .expect("plugin should spawn");

    expect_render(&mut plugin, &format!("hello {} {} 24x80", name, mode_name));

    plugin.send(OutgoingMessage::key("a", true));
    expect_render(&mut plugin, &format!("key {} a", name));

    plugin.send(OutgoingMessage::key("é", true));
    expect_render(&mut plugin, &format!("key {} é", name));

    plugin.send(OutgoingMessage::resize(30, 100));
    expect_render(&mut plugin, &format!("resize {} 30x100", name));

    plugin.send(OutgoingMessage::click(3));
    expect_render(&mut plugin, &format!("click {} 3", name));

    plugin.send(OutgoingMessage::focus(false));
    expect_render(&mut plugin, &format!("focus {} false", name));

    plugin.send(OutgoingMessage::key("s", true));
    let status = wait_for(&mut plugin, "status message", |m| {
        matches!(m, IncomingMessage::Status { .. })
    });
    assert_eq!(
        status,
        IncomingMessage::Status {
            message: format!("status {}", name),
            is_error: true
        }
    );

    plugin.send(OutgoingMessage::key("o", true));
    let opened = wait_for(&mut plugin, "open_file message", |m| {
        matches!(m, IncomingMessage::OpenFile { .. })
    });
    assert_eq!(
        opened,
        IncomingMessage::OpenFile {
            path: "/tmp/cera-plugin-test.txt".to_string()
        }
    );

    plugin.send(OutgoingMessage::key("q", true));
    wait_for(&mut plugin, "close message", |m| {
        matches!(m, IncomingMessage::Close)
    });

    plugin.send(OutgoingMessage::shutdown());
    let started = Instant::now();
    while plugin.is_alive() {
        assert!(
            started.elapsed() < WAIT_LIMIT,
            "{} plugin did not exit after shutdown",
            name
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn run_language(language: &PluginLanguage) {
    if !tool_available(language.tool) {
        assert!(
            std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
            "{} is required but not available",
            language.tool
        );
        eprintln!("SKIP {}: {} not available", language.name, language.tool);
        return;
    }
    let dir = TempDir(std::env::temp_dir().join(format!(
        "cera_plugin_lang_{}_{}",
        language.name,
        std::process::id()
    )));
    std::fs::create_dir_all(&dir.0).expect("temp dir");
    let (command, args) = prepare(language, &dir.0);
    exercise(language.name, &command, &args, DisplayMode::Fullscreen);
    exercise(language.name, &command, &args, DisplayMode::Panel);
}

#[test]
fn plugin_written_in_python_speaks_the_protocol() {
    run_language(&PYTHON);
}

#[test]
fn plugin_written_in_javascript_speaks_the_protocol() {
    run_language(&JAVASCRIPT);
}

#[test]
fn plugin_written_in_c_speaks_the_protocol() {
    run_language(&C);
}

#[test]
fn plugin_written_in_cpp_speaks_the_protocol() {
    run_language(&CPP);
}

#[test]
fn plugin_written_in_rust_speaks_the_protocol() {
    run_language(&RUST);
}
