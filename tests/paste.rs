use std::process::{Command, Stdio};

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

signal.alarm(170)
binary, workdir, home = sys.argv[1], sys.argv[2], sys.argv[3]
ORIGINAL = "alpha beta\nsecond line\nthird line\nfourth\n"
START = b"\x1b[200~"
END = b"\x1b[201~"
results = []


def start(name="f.txt"):
    pid, fd = pty.fork()
    if pid == 0:
        os.environ.update(TERM="xterm-256color", HOME=home, LANG="en_US.UTF-8")
        os.environ.pop("XDG_CONFIG_HOME", None)
        os.chdir(workdir)
        os.execv(binary, [binary, name])
        os._exit(1)
    fcntl.ioctl(fd, termios.TIOCSWINSZ, struct.pack("HHHH", 24, 100, 0, 0))
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
    return out


def stop(pid):
    try:
        os.kill(pid, signal.SIGKILL)
        os.waitpid(pid, 0)
    except OSError:
        pass


def send_all(fd, data):
    for k in range(0, len(data), 4096):
        os.write(fd, data[k:k + 4096])
        time.sleep(0.001)


def scenario(name, expected, body, settle=0.4, original=ORIGINAL):
    with open(os.path.join(workdir, "f.txt"), "w", encoding="utf-8") as f:
        f.write(original)
    pid, fd = start()
    drain(fd, 0.7)
    os.write(fd, b"\x1b[B")
    drain(fd, 0.15)
    os.write(fd, b"\x1b[C" * 3)
    drain(fd, 0.15)
    body(fd)
    drain(fd, settle)
    os.write(fd, b"\x13")
    changes = callable(expected) or expected != original
    deadline = time.time() + (30 if changes else 0.6)
    while time.time() < deadline:
        drain(fd, 0.05)
        with open(os.path.join(workdir, "f.txt"), "r", encoding="utf-8") as f:
            if changes and f.read() != original:
                drain(fd, 0.2)
                break
    stop(pid)
    with open(os.path.join(workdir, "f.txt"), "r", encoding="utf-8") as f:
        actual = f.read()
    if callable(expected):
        verdict = expected(actual)
    else:
        verdict = actual == expected
    results.append(("OK " if verdict else "FAIL ") + name + ("" if verdict else ": " + repr(actual[:300])))


def paste(payload):
    return lambda fd: send_all(fd, START + payload + END)


MID = "alpha beta\nsecX1\nX2\n  X3ond line\nthird line\nfourth\n"
scenario("lf", MID, paste(b"X1\nX2\n  X3"))
scenario("crlf", MID, paste(b"X1\r\nX2\r\n  X3"))
scenario("cr", MID, paste(b"X1\rX2\r  X3"))
scenario("trailing_newline_and_tab", "alpha beta\nseca\n\tb\nond line\nthird line\nfourth\n", paste(b"a\n\tb\n"))
scenario("control_chars_are_not_executed", "alpha beta\nsecPQRSond line\nthird line\nfourth\n", paste(b"P\x11Q\x13R\x01S"))
scenario("utf8", "alpha beta\nsec\u00e9\u20ac\U0001F600\n\u00fcond line\nthird line\nfourth\n", paste("\u00e9\u20ac\U0001F600\n\u00fc".encode("utf-8")))


def split_markers(fd):
    os.write(fd, b"\x1b[2")
    time.sleep(0.15)
    os.write(fd, b"00~M1\nM2\x1b[20")
    time.sleep(0.15)
    os.write(fd, b"1~")


scenario("markers_split_across_reads", "alpha beta\nsecM1\nM2ond line\nthird line\nfourth\n", split_markers)


def undo_once(fd):
    send_all(fd, START + b"U1\nU2\nU3" + END)
    drain(fd, 0.5)
    os.write(fd, b"\x1a")


scenario("single_undo_step", ORIGINAL, undo_once)


def replace_selection(fd):
    os.write(fd, b"\x1b[1;5C" * 4)
    drain(fd, 0.2)
    send_all(fd, START + b"Z1\nZ2" + END)


scenario("replaces_selection", "alpha beta\nsecZ1\nZ2line\nthird line\nfourth\n", replace_selection)


def type_after(fd):
    send_all(fd, START + b"P1\nP2" + END)
    drain(fd, 0.3)
    os.write(fd, b"Q")


scenario("typing_after_paste", "alpha beta\nsecP1\nP2Qond line\nthird line\nfourth\n", type_after)

big_lines = [("line %05d " % i) + "x" * 50 for i in range(5000)]
big = "\n".join(big_lines).encode("ascii")


def big_check(actual):
    rows = actual.split("\n")
    return (
        len(rows) == 5004
        and rows[1] == "sec" + big_lines[0]
        and rows[4999].startswith("line 04998")
        and rows[5000] == big_lines[4999] + "ond line"
        and rows[-2] == "fourth"
    )


started = time.time()
scenario("big_paste_5000_lines", big_check, paste(big), settle=1.0)
elapsed = time.time() - started
results.append(("OK " if elapsed < 40 else "FAIL ") + "big_paste_is_fast (%.1fs)" % elapsed)

with open(os.path.join(workdir, "g.txt"), "w") as f:
    f.write("one\n")
pid, fd = start("g.txt")
first = drain(fd, 1.2)
os.write(fd, b"\x11")
last = drain(fd, 1.0)
stop(pid)
results.append(("OK " if b"\x1b[?2004h" in first else "FAIL ") + "enables_bracketed_paste")
results.append(("OK " if b"\x1b[?2004l" in last else "FAIL ") + "disables_bracketed_paste_on_exit")

for line in results:
    print(line)
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
fn multiline_paste_in_the_middle_of_a_file_is_inserted_as_one_block() {
    if !python_available() {
        assert!(
            std::env::var_os("CERA_REQUIRE_PLUGIN_LANGS").is_none(),
            "python3 is required but not available"
        );
        eprintln!("SKIP: python3 not available");
        return;
    }
    let base = std::env::temp_dir().join(format!("cera_paste_{}", std::process::id()));
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
        "lf",
        "crlf",
        "cr",
        "trailing_newline_and_tab",
        "control_chars_are_not_executed",
        "utf8",
        "markers_split_across_reads",
        "single_undo_step",
        "replaces_selection",
        "typing_after_paste",
        "big_paste_5000_lines",
        "enables_bracketed_paste",
        "disables_bracketed_paste_on_exit",
    ] {
        assert!(
            stdout.lines().any(|l| l == format!("OK {expected}")),
            "missing result for {expected}:\n{stdout}"
        );
    }
}
