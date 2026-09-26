use std::io::{self, Read, Write};
use std::os::fd::AsFd;
use std::os::unix::io::RawFd;
use std::sync::atomic::{AtomicBool, Ordering};

use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use nix::sys::termios::{
    self, InputFlags, LocalFlags, OutputFlags, SetArg, SpecialCharacterIndices,
};

pub const NO_KEY: i32 = -2;
pub const BACKSPACE: i32 = 127;
pub const ARROW_LEFT: i32 = 1000;
pub const ARROW_RIGHT: i32 = 1001;
pub const ARROW_UP: i32 = 1002;
pub const ARROW_DOWN: i32 = 1003;
pub const DEL_KEY: i32 = 1004;
pub const HOME_KEY: i32 = 1005;
pub const END_KEY: i32 = 1006;
pub const PAGE_UP: i32 = 1007;
pub const PAGE_DOWN: i32 = 1008;
pub const MOUSE_EVENT: i32 = 1009;
pub const CTRL_ARROW_LEFT: i32 = 1010;
pub const CTRL_ARROW_RIGHT: i32 = 1011;
pub const CTRL_ARROW_UP: i32 = 1012;
pub const CTRL_ARROW_DOWN: i32 = 1013;

const CODEPOINT_BASE: i32 = 0x110000;

pub fn is_encoded_codepoint(key: i32) -> bool {
    key >= CODEPOINT_BASE
}

pub fn decode_codepoint(key: i32) -> char {
    char::from_u32((key - CODEPOINT_BASE) as u32).unwrap_or('\u{FFFD}')
}

fn encode_codepoint_as_key(c: char) -> i32 {
    CODEPOINT_BASE + c as i32
}

#[derive(Clone, Copy, Default)]
pub struct MouseEvent {
    pub button: i32,
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
    pub is_wheel: bool,
    pub wheel_delta: i32,
    pub is_drag: bool,
}

static RESIZE_PENDING: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigwinch(_: i32) {
    RESIZE_PENDING.store(true, Ordering::SeqCst);
}

pub struct Terminal {
    orig_termios: termios::Termios,
    raw_mode_enabled: bool,
    last_mouse: MouseEvent,
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        let stdin_fd = std::io::stdin();
        let orig_termios = termios::tcgetattr(stdin_fd.as_fd()).map_err(io::Error::other)?;

        let mut term = Terminal {
            orig_termios: orig_termios.clone(),
            raw_mode_enabled: false,
            last_mouse: MouseEvent::default(),
        };

        term.enable_raw_mode()?;
        term.write(b"\x1b[?1049h");

        Ok(term)
    }

    fn enable_raw_mode(&mut self) -> io::Result<()> {
        let stdin_fd = std::io::stdin();
        let mut raw = self.orig_termios.clone();

        raw.input_flags.remove(
            InputFlags::BRKINT
                | InputFlags::ICRNL
                | InputFlags::INPCK
                | InputFlags::ISTRIP
                | InputFlags::IXON,
        );
        raw.output_flags.remove(OutputFlags::OPOST);
        raw.local_flags
            .remove(LocalFlags::ECHO | LocalFlags::ICANON | LocalFlags::IEXTEN | LocalFlags::ISIG);
        raw.control_chars[SpecialCharacterIndices::VMIN as usize] = 0;
        raw.control_chars[SpecialCharacterIndices::VTIME as usize] = 1;

        termios::tcsetattr(stdin_fd.as_fd(), SetArg::TCSAFLUSH, &raw).map_err(io::Error::other)?;

        self.raw_mode_enabled = true;
        Ok(())
    }

    fn disable_raw_mode(&mut self) {
        if self.raw_mode_enabled {
            let stdin_fd = std::io::stdin();
            let _ = termios::tcsetattr(stdin_fd.as_fd(), SetArg::TCSAFLUSH, &self.orig_termios);
            self.raw_mode_enabled = false;
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        let _ = io::stdout().write_all(buf);
        let _ = io::stdout().flush();
    }

    pub fn enable_mouse_reporting(&mut self) {
        self.write(b"\x1b[?1002h\x1b[?1006h");
    }

    pub fn disable_mouse_reporting(&mut self) {
        self.write(b"\x1b[?1006l\x1b[?1002l");
    }

    /// Restaure completement l'etat du terminal avant de quitter le
    /// programme : desactive le rapport souris, quitte l'ecran alternatif,
    /// puis restaure le mode terminal d'origine (raw mode desactive).
    /// A appeler explicitement avant tout `std::process::exit`, car
    /// `process::exit` ne declenche jamais `Drop` : sans cet appel, le
    /// terminal reste en mode rapport souris (la souris ne fonctionne
    /// plus normalement dans le shell apres avoir quitte Cera) et en mode
    /// raw.
    pub fn shutdown(&mut self) {
        self.disable_mouse_reporting();
        self.write(b"\x1b[?1049l");
        self.disable_raw_mode();
    }

    pub fn last_mouse_event(&self) -> MouseEvent {
        self.last_mouse
    }

    pub fn install_resize_handler() {
        let handler = SigHandler::Handler(handle_sigwinch);
        let action = SigAction::new(handler, SaFlags::SA_RESTART, SigSet::empty());
        unsafe {
            let _ = signal::sigaction(Signal::SIGWINCH, &action);
        }
    }

    pub fn consume_resize_flag() -> bool {
        RESIZE_PENDING.swap(false, Ordering::SeqCst)
    }

    pub fn get_window_size(&mut self) -> io::Result<(u16, u16)> {
        match get_window_size_ioctl() {
            Some((rows, cols)) => Ok((rows, cols)),
            None => self.get_window_size_via_cursor(),
        }
    }

    fn get_window_size_via_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.write(b"\x1b[999C\x1b[999B");
        self.get_cursor_position()
    }

    fn read_byte_raw(&self) -> io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        match io::stdin().read(&mut buf) {
            Ok(0) => Ok(None),
            Ok(_) => Ok(Some(buf[0])),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn read_byte_blocking(&self) -> io::Result<u8> {
        loop {
            if let Some(b) = self.read_byte_raw()? {
                return Ok(b);
            }
        }
    }

    fn get_cursor_position(&mut self) -> io::Result<(u16, u16)> {
        self.write(b"\x1b[6n");
        let mut resp = Vec::new();
        loop {
            let b = self.read_byte_blocking()?;
            if b == b'R' {
                break;
            }
            resp.push(b);
            if resp.len() > 31 {
                break;
            }
        }
        let s = String::from_utf8_lossy(&resp);
        let s = s.trim_start_matches('\u{1b}').trim_start_matches('[');
        let mut parts = s.split(';');
        let rows: u16 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(24);
        let cols: u16 = parts.next().and_then(|p| p.parse().ok()).unwrap_or(80);
        Ok((rows, cols))
    }

    pub fn read_key(&mut self) -> io::Result<i32> {
        let first = match self.read_byte_raw()? {
            Some(b) => b,
            None => return Ok(NO_KEY),
        };

        if first == 0x1b {
            let seq0 = match self.read_byte_raw()? {
                Some(b) => b,
                None => return Ok(0x1b),
            };
            let seq1 = match self.read_byte_raw()? {
                Some(b) => b,
                None => return Ok(0x1b),
            };

            if seq0 == b'[' {
                if seq1 == b'<' {
                    return self.read_sgr_mouse();
                }
                if seq1.is_ascii_digit() {
                    let mut num = String::new();
                    num.push(seq1 as char);
                    let mut next = self.read_byte_blocking()?;
                    while next.is_ascii_digit() {
                        num.push(next as char);
                        next = self.read_byte_blocking()?;
                    }
                    if next == b'~' {
                        let code: i32 = num.parse().unwrap_or(0);
                        return Ok(match code {
                            1 | 7 => HOME_KEY,
                            3 => DEL_KEY,
                            4 | 8 => END_KEY,
                            5 => PAGE_UP,
                            6 => PAGE_DOWN,
                            _ => 0x1b,
                        });
                    }
                    if next == b';' {
                        let mut modifier_str = String::new();
                        let mut b2 = self.read_byte_blocking()?;
                        while b2.is_ascii_digit() {
                            modifier_str.push(b2 as char);
                            b2 = self.read_byte_blocking()?;
                        }
                        let modifier: i32 = modifier_str.parse().unwrap_or(1);
                        let ctrl_held = modifier == 5 || modifier == 6;
                        return Ok(match b2 {
                            b'A' => {
                                if ctrl_held {
                                    CTRL_ARROW_UP
                                } else {
                                    ARROW_UP
                                }
                            }
                            b'B' => {
                                if ctrl_held {
                                    CTRL_ARROW_DOWN
                                } else {
                                    ARROW_DOWN
                                }
                            }
                            b'C' => {
                                if ctrl_held {
                                    CTRL_ARROW_RIGHT
                                } else {
                                    ARROW_RIGHT
                                }
                            }
                            b'D' => {
                                if ctrl_held {
                                    CTRL_ARROW_LEFT
                                } else {
                                    ARROW_LEFT
                                }
                            }
                            b'H' => HOME_KEY,
                            b'F' => END_KEY,
                            _ => 0x1b,
                        });
                    }
                    return Ok(0x1b);
                }
                return Ok(match seq1 {
                    b'A' => ARROW_UP,
                    b'B' => ARROW_DOWN,
                    b'C' => ARROW_RIGHT,
                    b'D' => ARROW_LEFT,
                    b'H' => HOME_KEY,
                    b'F' => END_KEY,
                    _ => 0x1b,
                });
            } else if seq0 == b'O' {
                return Ok(match seq1 {
                    b'H' => HOME_KEY,
                    b'F' => END_KEY,
                    _ => 0x1b,
                });
            }
            return Ok(0x1b);
        }

        let len = utf8_sequence_length(first);
        if len == 1 {
            return Ok(first as i32);
        }

        let mut bytes = vec![first];
        for _ in 1..len {
            bytes.push(self.read_byte_blocking()?);
        }
        match std::str::from_utf8(&bytes) {
            Ok(s) => {
                let c = s.chars().next().unwrap_or('\u{FFFD}');
                Ok(encode_codepoint_as_key(c))
            }
            Err(_) => Ok(encode_codepoint_as_key('\u{FFFD}')),
        }
    }

    fn read_sgr_mouse(&mut self) -> io::Result<i32> {
        let mut values = [0i32; 3];
        let mut value_idx = 0usize;
        let mut final_ch = b'M';

        loop {
            let b = self.read_byte_blocking()?;
            if b == b';' {
                value_idx += 1;
                if value_idx > 2 {
                    break;
                }
                continue;
            }
            if b == b'M' || b == b'm' {
                final_ch = b;
                break;
            }
            if b.is_ascii_digit() && value_idx <= 2 {
                values[value_idx] = values[value_idx] * 10 + (b - b'0') as i32;
            }
        }

        self.last_mouse = MouseEvent {
            button: values[0] & 0x3,
            is_wheel: (values[0] & 0x40) != 0,
            is_drag: (values[0] & 0x20) != 0,
            wheel_delta: if (values[0] & 0x40) != 0 {
                if (values[0] & 0x1) != 0 {
                    1
                } else {
                    -1
                }
            } else {
                0
            },
            x: values[1] - 1,
            y: values[2] - 1,
            pressed: final_ch == b'M',
        };

        Ok(MOUSE_EVENT)
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn utf8_sequence_length(lead: u8) -> usize {
    if lead & 0x80 == 0x00 {
        1
    } else if lead & 0xE0 == 0xC0 {
        2
    } else if lead & 0xF0 == 0xE0 {
        3
    } else if lead & 0xF8 == 0xF0 {
        4
    } else {
        1
    }
}

fn get_window_size_ioctl() -> Option<(u16, u16)> {
    #[repr(C)]
    struct WinSize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    let fd: RawFd = 1;
    let mut ws = WinSize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    let ret = unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws as *mut WinSize) };

    if ret == -1 || ws.ws_col == 0 {
        return None;
    }

    Some((ws.ws_row, ws.ws_col))
}

pub fn die(context: &str) -> ! {
    let _ = io::stdout().write_all(b"\x1b[?1049l");
    let _ = io::stdout().flush();
    eprintln!("{}: {}", context, io::Error::last_os_error());
    std::process::exit(1);
}
