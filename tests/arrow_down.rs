use std::process::Command;

const PTY_SCRIPT: &str = r##"
import fcntl
import os
import pty
import select
import signal
import struct
import sys
import termios
import time

signal.alarm(120)
binary, workdir, home = sys.argv[1], sys.argv[2], sys.argv[3]
ORIGINAL = "one\ntwo\n"
DOWN = b"\x1b[B"
RIGHT = b"\x1b[C"
PAGE_DOWN = b"\x1b[6~"
results = []


def start():
    pid, fd = pty.fork()
    if pid == 0:
        os.environ.update(TERM="xterm-256color", HOME=home, LANG="en_US.UTF-8")
        os.environ.pop("XDG_CONFIG_HOME", None)
        os.chdir(workdir)
        os.execv(binary, [binary, "f.txt"])
        os._exit(1)
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 100, 0, 0))
    return pid, fd


def drain(fd, duration):
    end = time.time() + duration
    while time.time() < end:
        ready, _, _ = select.select([fd], [], [], 0.02)
        if ready:
            try:
                os.read(fd, 65536)
            except OSError:
                break


def stop(pid):
    try:
        os.kill(pid, signal.SIGKILL)
        os.waitpid(pid, 0)
    except OSError:
        pass


def scenario(name, keys, expected):
    path = os.path.join(workdir, "f.txt")
    with open(path, "w", encoding="utf-8") as f:
        f.write(ORIGINAL)
    pid, fd = start()
    drain(fd, 0.7)
    for key in keys:
        os.write(fd, key)
        drain(fd, 0.15)
    os.write(fd, b"\x13")
    deadline = time.time() + (10 if expected != ORIGINAL else 0.6)
    while time.time() < deadline:
        drain(fd, 0.05)
        with open(path, "r", encoding="utf-8") as f:
            if expected != ORIGINAL and f.read() != ORIGINAL:
                drain(fd, 0.2)
                break
    stop(pid)
    with open(path, "r", encoding="utf-8") as f:
        actual = f.read()
    verdict = actual == expected
    results.append(("OK " if verdict else "FAIL ") + name + ("" if verdict else ": " + repr(actual)))


scenario("down_on_last_line_creates_line", [DOWN, DOWN, b"x"], "one\ntwo\nx\n")
scenario("down_in_the_middle_does_not_create", [DOWN, b"x"], "one\nxtwo\n")
scenario("page_down_does_not_create", [PAGE_DOWN, PAGE_DOWN, b"y"], "one\nytwo\n")
scenario("created_line_can_be_undone", [DOWN, DOWN, b"\x1a", b"x"], "one\ntwox\n")
scenario("last_line_is_not_split", [DOWN, RIGHT, DOWN, b"x"], "one\ntwo\nx\n")
scenario("two_downs_create_two_lines", [DOWN, DOWN, DOWN, b"x"], "one\ntwo\n\nx\n")

print("\n".join(results))
"##;

#[test]
fn arrow_down_on_last_line_appends_a_line() {
    if Command::new("python3").arg("--version").output().is_err() {
        assert!(
            std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
            "python3 is required but not available"
        );
        eprintln!("SKIP: python3 not available");
        return;
    }
    let base = std::env::temp_dir().join(format!("cera_arrow_down_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let workdir = base.join("work");
    let home = base.join("home");
    std::fs::create_dir_all(&workdir).unwrap();
    std::fs::create_dir_all(&home).unwrap();

    let output = Command::new("python3")
        .arg("-c")
        .arg(PTY_SCRIPT)
        .arg(env!("CARGO_BIN_EXE_cera"))
        .arg(&workdir)
        .arg(&home)
        .output()
        .expect("python3 should start");
    let _ = std::fs::remove_dir_all(&base);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "pty script failed: {}\n{}",
        String::from_utf8_lossy(&output.stderr),
        stdout
    );
    let failures: Vec<&str> = stdout.lines().filter(|l| l.starts_with("FAIL")).collect();
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
    for expected in [
        "down_on_last_line_creates_line",
        "down_in_the_middle_does_not_create",
        "page_down_does_not_create",
        "created_line_can_be_undone",
        "last_line_is_not_split",
        "two_downs_create_two_lines",
    ] {
        assert!(
            stdout.lines().any(|l| l == format!("OK {expected}")),
            "missing result for {expected}:\n{stdout}"
        );
    }
}
