use std::io;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct AltScreen;

impl AltScreen {
    pub fn enter() -> Self {
        print!("\x1b[?1049h\x1b[?25l");
        let _ = io::stdout().flush();
        Self
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        print!("\x1b[?25h\x1b[?1049l");
        let _ = io::stdout().flush();
    }
}
