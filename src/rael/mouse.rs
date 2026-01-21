use crate::rael::Rael;
use rustix::io::dup;
use rustix::io::read;
use std::io;
use std::io::Write;
use std::os::fd::AsFd;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEvent {
    Press {
        button: MouseButton,
        x: usize,
        y: usize,
    },
    Release {
        button: MouseButton,
        x: usize,
        y: usize,
    },
    Move {
        x: usize,
        y: usize,
    },
    Scroll {
        direction: ScrollDirection,
        x: usize,
        y: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Other(u8), // in case platform defines others
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
}

// i sadly vibecoded ts, but if u ask me smt abt ts code i can surly explain it
pub fn parse_sgr_mouse(seq: &str) -> Option<MouseEvent> {
    let (seq, is_release) = if let Some(rest) = seq.strip_suffix('M') {
        (rest, false)
    } else if let Some(rest) = seq.strip_suffix('m') {
        (rest, true)
    } else {
        return None;
    };

    let seq = seq.strip_prefix("\x1b[<")?;
    let parts: Vec<_> = seq.split(';').collect();
    if parts.len() != 3 {
        return None;
    }

    let b: u8 = parts[0].parse().ok()?;
    let x: usize = parts[1].parse().ok()?;
    let y: usize = parts[2].parse().ok()?;

    // Scroll
    if b & 0b1000000 != 0 {
        let direction = if b & 1 != 0 {
            ScrollDirection::Down
        } else {
            ScrollDirection::Up
        };
        return Some(MouseEvent::Scroll { direction, x, y });
    }

    // Move
    if b & 0b0100000 != 0 {
        return Some(MouseEvent::Move { x, y });
    }

    // Buttons
    let button = match b & 0b11 {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        other => MouseButton::Other(other),
    };

    if is_release {
        Some(MouseEvent::Release { button, x, y })
    } else {
        Some(MouseEvent::Press { button, x, y })
    }
}

impl Rael {
    pub fn enable_mouse(&mut self) -> Receiver<MouseEvent> {
        let (send, recv) = channel::<MouseEvent>();
        print!("\x1b[?1003h\x1b[?1006h");
        let _ = io::stdout().flush();
        let mut buf = [0u8; 32];
        let raw = dup(self.fd.as_fd()).unwrap();
        thread::spawn(move || {
            loop {
                let n = read(&raw, &mut buf).unwrap();
                if n == 0 {
                    continue;
                }
                if let Ok(s) = str::from_utf8(&buf[..n])
                    && s.contains("\x1b[<")
                    && let Some(evt) = parse_sgr_mouse(s)
                {
                    send.send(evt).unwrap()
                }
            }
        });
        self.mouse_enabled = true;
        recv
    }
}
