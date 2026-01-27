use crossterm::event::{
    EnableMouseCapture, EventStream, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::{
    cursor::{Hide, Show},
    event::EnableFocusChange,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, supports_keyboard_enhancement, window_size,
        BeginSynchronizedUpdate, DisableLineWrap, EnableLineWrap, EndSynchronizedUpdate,
        EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
};
use rayon::prelude::*;
use std::io::{self, Stdout, Write};

use crate::rael::input::Input;

mod input;

const MAX: usize = 512;

/// RGB color
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    /// Red channel (0–255)
    pub r: u8,
    /// Green channel (0–255)
    pub g: u8,
    /// Blue channel (0–255)
    pub b: u8,
}

impl Color {
    /// Create a new color from RGB values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageAsset<const W: usize, const H: usize> {
    pub pixels: [[u8; W]; H],     // pixel indices into the palette
    pub colors: &'static [Color], // palette, slice instead of const generic
}

/// Main terminal renderer
pub struct Rael {
    /// Terminal width
    pub widht: u16,
    /// Terminal height (doubled for pixel aspect)
    pub height: u16,
    /// Current pixel buffer (stores color indices)
    pub pixels: [[u8; MAX]; MAX],
    /// Z-buffer for depth handling
    pub z_buffer: [[u8; MAX]; MAX],
    /// List of unique colors used
    pub colors: Vec<Color>,
    /// Stdout handle for rendering
    pub stdout: Stdout,
    /// Previous frame for diff rendering
    pub old: [[u8; MAX]; MAX],
    /// Input handler
    pub inputs: Input,
    /// char for custom rendering
    pub chars: [[char; MAX]; MAX / 2],
}

impl Rael {
    /// Initialize Rael and set up terminal
    ///
    /// # Parameters
    /// - `stdout`: terminal output handle
    /// - `title`: window title
    ///
    /// # Errors
    /// Returns an error if terminal doesn't support enhanced keyboard protocols
    pub fn new(mut stdout: Stdout, title: &str) -> Result<Self, io::Error> {
        if !supports_keyboard_enhancement().unwrap() {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Terminal doesn't support Kitty protocols, required for rendering",
            ));
        };

        let _ = enable_raw_mode();
        execute!(
            stdout,
            EnterAlternateScreen,
            DisableLineWrap,
            EnableFocusChange,
            EnableMouseCapture,
            SetTitle::<&str>(title),
            Hide,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
        )?;

        let win = window_size().unwrap();
        let reader = EventStream::new();

        Ok(Rael {
            widht: win.columns,
            height: win.rows * 2,
            pixels: [[0; MAX]; MAX],
            z_buffer: [[0; MAX]; MAX],
            colors: vec![Color::new(0, 0, 0)],
            stdout,
            old: [[1; MAX]; MAX],
            inputs: Input::new(reader),
            chars: [[' '; MAX]; MAX / 2],
        })
    }

    /// Get the index of a color, inserting it if new
    ///
    /// # Parameters
    /// - `color`: the color to find or insert
    ///
    /// # Returns
    /// The index of the color in the color palette
    fn get_or_insert_color(&mut self, color: Color) -> u8 {
        if let Some(i) = self.colors.par_iter().position_any(|&mraow| mraow == color) {
            i as u8
        } else {
            let i = self.colors.len();
            self.colors.push(color);
            i as u8
        }
    }

    /// Set a pixel at a specific position with depth and color
    ///
    /// # Parameters
    /// - `x`, `y`: pixel coordinates
    /// - `z`: depth for z-buffer
    /// - `color`: pixel color
    ///
    /// # Panics
    /// Panics if `x` or `y` exceeds MAX
    pub fn set_pixel(&mut self, x: usize, y: usize, z: u8, color: Color, char: Option<char>) {
        if x > MAX || y > MAX {
            panic!("y={y} and x={x}, one of them exceeds MAX:{MAX}");
        }
        if self.z_buffer[y][x] <= z {
            self.pixels[y][x] = self.get_or_insert_color(color);
            self.z_buffer[y][x] = z;
            self.chars[y / 2][x] = char.unwrap_or(' ');
        }
    }

    pub fn set_image<const W: usize, const H: usize>(
        &mut self,
        image: ImageAsset<W, H>,
        pos: (usize, usize, u8),
    ) {
        let (ox, oy, oz) = pos;

        for y in 0..H {
            let ty = oy + y;
            if ty >= H {
                continue;
            }

            for x in 0..W {
                let tx = ox + x;
                let color_index = image.pixels[y][x] as usize;

                if color_index >= image.colors.len() - 1 {
                    continue;
                }

                let color = image.colors[color_index];
                self.set_pixel(tx, ty, oz, color, None);
            }
        }
    }

    /// Clear the pixel buffer while saving the previous frame
    ///
    /// Use this before rendering a new frame
    pub fn clear(&mut self) {
        self.old = self.pixels;
        self.pixels = [[0; MAX]; MAX];
        self.z_buffer = [[0; MAX]; MAX];
        self.chars = [[' '; MAX]; MAX / 2]
    }

    /// Clear all stored colors except black
    pub fn clear_colors(&mut self) {
        self.colors = vec![Color::new(0, 0, 0)];
    }

    /// Render the current frame to the terminal
    ///
    /// Only updates pixels that changed since last frame
    pub async fn render(&mut self) -> io::Result<()> {
        execute!(self.stdout, BeginSynchronizedUpdate)?;
        let mut buffer = String::new();

        for y in (0..self.height as usize).step_by(2) {
            for x in 0..self.widht as usize {
                buffer.push_str(&format!("\u{1b}[{};{}H", (y + 1).div_ceil(2), x + 1));

                let top = self.pixels[y][x];
                let bottom = if y + 1 < self.height.into() {
                    self.pixels[y + 1][x]
                } else {
                    top
                };

                let otop = self.old[y][x];
                let obottom = if y + 1 < self.height.into() {
                    self.old[y + 1][x]
                } else {
                    otop
                };

                if top == otop && bottom == obottom {
                    continue;
                }

                let top = self.colors[top as usize];
                let bottom = self.colors[bottom as usize];
                if top == bottom {
                    buffer.push_str(&format!("\u{1b}[48;2;{};{};{}m ", top.r, top.g, top.b));
                } else if self.chars[y / 2][x] != ' ' {
                    buffer.push_str(&format!(
                        "\u{1b}[48;2;{};{};{}m\u{1b}[38;2;{};{};{}m{}",
                        top.r,
                        top.g,
                        top.b,
                        bottom.r,
                        bottom.g,
                        bottom.b,
                        self.chars[y / 2][x]
                    ));
                } else {
                    buffer.push_str(&format!(
                        "\u{1b}[48;2;{};{};{}m\u{1b}[38;2;{};{};{}m▄",
                        top.r, top.g, top.b, bottom.r, bottom.g, bottom.b
                    ));
                }
            }
        }

        execute!(self.stdout, EndSynchronizedUpdate)?;
        self.stdout.write_all(buffer.as_bytes())?;
        Ok(())
    }

    pub fn render_custom(&mut self, string: String) -> io::Result<()> {
        self.stdout.write_all(string.as_bytes())?;
        Ok(())
    }
}

impl Drop for Rael {
    /// Restore the terminal to its original state when Rael is dropped
    fn drop(&mut self) {
        let _ = execute!(
            self.stdout,
            PopKeyboardEnhancementFlags,
            EndSynchronizedUpdate,
            EnableLineWrap,
            LeaveAlternateScreen,
            Show
        );
        let _ = disable_raw_mode();
        let _ = self.stdout.flush();
    }
}
