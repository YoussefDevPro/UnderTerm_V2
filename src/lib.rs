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
use std::thread;

use std::sync::atomic::{AtomicBool, Ordering};

static RESIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone)]
pub struct Rael {
    pub widht: u16,
    pub height: u16,
    fd: rustix::fd::BorrowedFd<'static>,
    original: Termios,
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
        let _ = tcsetattr(self.fd, OptionalActions::Now, &self.original); // enable raw mode

        println!("AS RAEL AS IT GETS!!! \n Muhihi hi hi-")
    }
}

impl Rael {
    pub fn new() -> Self {
        let fd: BorrowedFd = stdio::stdin();
        let mut original: Termios = tcgetattr(fd).unwrap();
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
        }
        Ok(())
    }

    pub fn set_wsize(&mut self, fd: impl AsFd) -> Result<(), rustix::io::Errno> {
        let ws: Winsize = tcgetwinsize(fd.as_fd())?;
        self.widht = ws.ws_col;
        self.height = ws.ws_row * 2;
        Ok(())
    }
}
