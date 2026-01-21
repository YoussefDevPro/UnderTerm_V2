use rustix::fd::AsFd;
use rustix::fd::BorrowedFd;
use rustix::stdio;
use rustix::termios::tcgetattr;
use rustix::termios::tcgetwinsize;
use rustix::termios::tcsetattr;
use rustix::termios::ControlModes;
use rustix::termios::Termios;
use rustix::termios::Winsize;
use rustix::termios::{InputModes, LocalModes, OptionalActions, OutputModes};

use signal_hook::consts::SIGWINCH;
use signal_hook::iterator::Signals;
use std::process::exit;
use std::thread;

use std::sync::atomic::{AtomicBool, Ordering};

use std::io::{stdout, Write};

mod alt_screen;
mod mouse;
mod screen;

pub use crate::rael::alt_screen::*;
pub use crate::rael::mouse::*;
pub use crate::rael::screen::*;

static RESIZED: AtomicBool = AtomicBool::new(false);
pub const MAX: usize = 512;

#[derive(Debug, Clone)]
pub struct Rael {
    pub widht: u16,
    pub height: u16,
    previous_screen: Option<OldScreen>,
    pub screen: Screen,
    fd: rustix::fd::BorrowedFd<'static>,
    original: Termios,
    alt: AltScreen,
    mouse_enabled: bool,
}

impl Default for Rael {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Rael {
    fn drop(&mut self) {
        self.original
            .local_modes
            .insert(LocalModes::ICANON | LocalModes::ISIG | LocalModes::IEXTEN | LocalModes::ECHO);
        self.original
            .input_modes
            .insert(InputModes::ICRNL | InputModes::IXON);
        self.original.output_modes.insert(OutputModes::OPOST);
        self.original.control_modes.insert(ControlModes::CS8);
        let _ = tcsetattr(self.fd, OptionalActions::Now, &self.original); // enable raw mode //
                                                                          // EDIT: DISABLE
        if self.mouse_enabled {
            print!("\x1b[?1003l\x1b[?1006l");
        }
    }
}

impl Rael {
    pub fn new() -> Self {
        let fd: BorrowedFd = stdio::stdin();
        let termioss = tcgetattr(fd);
        let mut original: Termios = match termioss {
            Ok(v) => v,
            Err(e) => {
                println!("something wrong happened\n{}", e);
                exit(0);
            }
        };
        original
            .local_modes
            .remove(LocalModes::ICANON | LocalModes::ISIG | LocalModes::IEXTEN | LocalModes::ECHO);
        original
            .input_modes
            .remove(InputModes::ICRNL | InputModes::IXON);
        original.output_modes.remove(OutputModes::OPOST);
        original.control_modes.remove(ControlModes::CS8);
        let _ = tcsetattr(fd, OptionalActions::Now, &original); // enable raw mode

        let size: Winsize = tcgetwinsize(fd).unwrap();

        Rael {
            widht: size.ws_col,
            height: size.ws_row * 2,
            fd,
            original,
            screen: Screen::new(),
            previous_screen: None,
            alt: AltScreen::enter(),
            mouse_enabled: false,
        }
    }

    pub fn setup_events(&self) {
        let mut signals = Signals::new([SIGWINCH]).unwrap();

        thread::spawn(move || {
            for _ in signals.forever() {
                RESIZED.store(true, Ordering::Relaxed);
            }
        });
    }

    pub fn update_wsize(&mut self) -> Result<(), rustix::io::Errno> {
        if RESIZED.swap(false, Ordering::Relaxed) {
            let _ = self.set_wsize(stdio::stdout().as_fd());
            self.previous_screen = None;
        }
        Ok(())
    }

    pub fn set_wsize(&mut self, fd: impl AsFd) -> Result<(), rustix::io::Errno> {
        let ws: Winsize = tcgetwinsize(fd.as_fd())?;
        self.widht = ws.ws_col;
        self.height = ws.ws_row * 2;
        Ok(())
    }

    pub fn render(&self) {
        print!(
            "{}",
            self.screen
                .render(self.widht, self.height, self.previous_screen)
        );
        stdout().flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.previous_screen = self.screen.clear();
    }
}
