use std::process::{Command, Stdio};

const PTY_SCRIPT: &str = r##"
import fcntl
import os
import pty
import re
import select
import signal
import struct
import sys
import termios
import time

signal.alarm(90)
binary, home = sys.argv[1], sys.argv[2]


def start():
    pid, fd = pty.fork()
    if pid == 0:
        os.environ.update(TERM="xterm-256color", HOME=home, LANG="en_US.UTF-8")
        os.environ.pop("XDG_CONFIG_HOME", None)
        os.execv(binary, [binary])
        os._exit(1)
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 80, 0, 0))
    return pid, fd


def drain(fd, duration):
    end = time.time() + duration
    out = b""
    while time.time() < end:
        ready, _, _ = select.select([fd], [], [], 0.02)
        if ready:
            try:
                chunk = os.read(fd, 65536)
            except OSError:
                break
            out += chunk
    return out.decode(errors="replace")


def plain(text):
    return re.sub(r"\x1b\[[0-9;?<]*[A-Za-z]", "", text)


def first_output_delay(fd, limit):
    started = time.time()
    while time.time() - started < limit:
        ready, _, _ = select.select([fd], [], [], 0.005)
        if ready:
            os.read(fd, 65536)
            return time.time() - started
    return None


def stop(pid):
    try:
        os.kill(pid, signal.SIGKILL)
        os.waitpid(pid, 0)
    except OSError:
        pass


pid, fd = start()
drain(fd, 1.0)
os.write(fd, b"\x04")
drain(fd, 0.6)
os.write(fd, b"\x1b")
latency = first_output_delay(fd, 4.0)
print("latency", "none" if latency is None else "%.3f" % latency)
time.sleep(0.15)
os.write(fd, b"abc")
print("typed_after_escape", "abc" in plain(drain(fd, 0.8)))
stop(pid)

pid, fd = start()
drain(fd, 1.0)
os.write(fd, b"ab")
drain(fd, 0.3)
os.write(fd, b"\x1b[D")
drain(fd, 0.3)
os.write(fd, b"X")
print("arrow_one_chunk", "aXb" in plain(drain(fd, 0.8)))
stop(pid)

pid, fd = start()
drain(fd, 1.0)
os.write(fd, b"ab")
drain(fd, 0.3)
os.write(fd, b"\x1b")
time.sleep(0.005)
os.write(fd, b"[D")
drain(fd, 0.3)
os.write(fd, b"X")
print("arrow_split", "aXb" in plain(drain(fd, 0.8)))
stop(pid)
"##;

fn python_available() -> bool {
    Command::new("python3")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[test]
fn escape_is_handled_immediately_and_does_not_swallow_input() {
    if !python_available() {
        assert!(
            std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
            "python3 is required but not available"
        );
        eprintln!("SKIP: python3 not available");
        return;
    }
    let home = std::env::temp_dir().join(format!("cera_escape_key_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&home);
    std::fs::create_dir_all(&home).unwrap();

    let output = Command::new("python3")
        .arg("-c")
        .arg(PTY_SCRIPT)
        .arg(env!("CARGO_BIN_EXE_cera"))
        .arg(&home)
        .output()
        .expect("python3 should start");
    let _ = std::fs::remove_dir_all(&home);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "pty script failed: {}\n{}",
        String::from_utf8_lossy(&output.stderr),
        stdout
    );
    let value = |key: &str| -> String {
        stdout
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key} ")))
            .unwrap_or_else(|| panic!("missing {key} in:\n{stdout}"))
            .to_string()
    };

    let latency: f64 = value("latency")
        .parse()
        .expect("the screen must redraw after Escape");
    assert!(latency < 0.6, "Escape took {latency}s to be handled");
    assert_eq!(value("typed_after_escape"), "True", "{stdout}");
    assert_eq!(value("arrow_one_chunk"), "True", "{stdout}");
    assert_eq!(value("arrow_split"), "True", "{stdout}");
}
