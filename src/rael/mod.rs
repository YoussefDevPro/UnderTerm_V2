use crossterm::cursor;
use crossterm::event::{
    EnableMouseCapture, EventStream, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags,
};
use crossterm::queue;
use crossterm::style::{Color as CrosstermColor, Print, SetBackgroundColor, SetForegroundColor};
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
use std::collections::HashMap;
use std::io::{self, Stdout, Write};

use crate::rael::input::Input;

mod input;

const MAX: usize = 512;

/// RGB color
#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
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
    pub pixels: [[u16; MAX]; MAX],
    /// Z-buffer for depth handling
    pub z_buffer: [[u8; MAX]; MAX],
    /// List of unique colors used
    pub colors: Vec<Color>,
    pub hash_colors: HashMap<Color, u16>,
    /// Stdout handle for rendering
    pub stdout: Stdout,
    /// Previous frame for diff rendering
    pub old: [[u16; MAX]; MAX],
    /// Input handler
    pub inputs: Input,
    /// char for custom rendering
    pub chars: [[char; MAX]; MAX],
    pub dirty_rows: [u128; 2],
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
        let mut hash_colors = HashMap::new();
        hash_colors.insert(Color { r: 0, g: 0, b: 0 }, 0);

        Ok(Rael {
            widht: win.columns,
            height: win.rows * 2,
            pixels: [[0; MAX]; MAX],
            z_buffer: [[0; MAX]; MAX],
            colors: vec![Color::new(0, 0, 0)],
            stdout,
            old: [[1; MAX]; MAX],
            inputs: Input::new(reader),
            chars: [[' '; MAX]; MAX],
            hash_colors,
            dirty_rows: [0; 2],
        })
    }

    fn get_or_insert_color(&mut self, color: Color) -> u16 {
        *self.hash_colors.entry(color).or_insert_with(|| {
            let new_index = self.colors.len() as u16;
            self.colors.push(color);
            new_index
        })
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
    pub fn set_pixel(&mut self, x: usize, y: usize, z: u8, color: Color) {
        if x > MAX || y > MAX {
            panic!("y={y} and x={x}, one of them exceeds MAX:{MAX}");
        }
        if self.z_buffer[y][x] <= z {
            self.pixels[y][x] = self.get_or_insert_color(color);
            self.z_buffer[y][x] = z;
            self.dirty_rows[(y / 2) / 128] |= 1 << ((y / 2) % 128);
        }
    }

    pub fn set_text(&mut self, x: usize, y: usize, z: u8, bg: Color, fg: Color, cchar: char) {
        if x > MAX || y > MAX {
            panic!("y={y} and x={x}, one of them exceeds MAX:{MAX}");
        }
        if self.z_buffer[y][x] <= z && y.is_multiple_of(2) {
            self.pixels[y][x] = self.get_or_insert_color(bg);
            self.z_buffer[y][x] = z;
            self.chars[y][x] = cchar;
            self.pixels[y + 1][x] = self.get_or_insert_color(fg);
            let term_y = if y % 2 == 0 { y / 2 } else { (y + 1) / 2 };
            self.dirty_rows[term_y / 128] |= 1 << (term_y % 128);
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
                self.set_pixel(tx, ty, oz, color);
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
        self.chars = [[' '; MAX]; MAX];
        self.dirty_rows = [0; 2];
    }

    /// Clear all stored colors except black
    pub fn clear_colors(&mut self) {
        self.colors = vec![Color::new(0, 0, 0)];
        let mut hash = HashMap::new();
        hash.insert(Color::new(0, 0, 0), 0);
        self.hash_colors = hash;
    }

    pub fn force_clear(&mut self) {
        // all those clears was not enough, so i made this >:3
        self.old = [[255; MAX]; MAX];
        self.pixels = [[0; MAX]; MAX];
        self.z_buffer = [[0; MAX]; MAX];
        self.chars = [[' '; MAX]; MAX];
        self.dirty_rows = [u128::MAX; 2];
        self.clear_colors();
    }

    pub async fn render(&mut self) -> io::Result<()> {
        queue!(self.stdout, BeginSynchronizedUpdate)?;

        for bucket in 0..2 {
            let mut yummy_bits = self.dirty_rows[bucket];

            if yummy_bits == 0 {
                continue;
            }

            while yummy_bits != 0 {
                let mini_bit = yummy_bits.trailing_zeros() as usize;
                let y = (bucket * 128) + mini_bit;

                let render_y = y * 2;

                queue!(self.stdout, cursor::MoveTo(0, (render_y / 2) as u16))?;

                for x in 0..self.widht as usize {
                    let top = self.pixels[render_y][x];
                    let bottom = if render_y + 1 < self.height.into() {
                        self.pixels[render_y + 1][x]
                    } else {
                        top
                    };

                    let old_top = self.old[render_y][x];
                    let old_bottom = if render_y + 1 < self.height.into() {
                        self.old[render_y + 1][x]
                    } else {
                        old_top
                    };

                    if top == old_top && bottom == old_bottom {
                        queue!(self.stdout, cursor::MoveRight(1))?;
                        continue;
                    }

                    let color_top = self.colors[top as usize];
                    let color_bottom = self.colors[bottom as usize];

                    if color_top == color_bottom {
                        queue!(
                            self.stdout,
                            SetBackgroundColor(CrosstermColor::Rgb {
                                r: color_top.r,
                                g: color_top.g,
                                b: color_top.b
                            }),
                            Print(" ")
                        )?;
                    } else if self.chars[render_y][x] != ' ' {
                        queue!(
                            self.stdout,
                            SetForegroundColor(CrosstermColor::Rgb {
                                r: color_bottom.r,
                                g: color_bottom.g,
                                b: color_bottom.b
                            }),
                            SetBackgroundColor(CrosstermColor::Rgb {
                                r: color_top.r,
                                g: color_top.g,
                                b: color_top.b
                            }),
                            Print(self.chars[render_y][x])
                        )?;
                    } else {
                        queue!(
                            self.stdout,
                            SetForegroundColor(CrosstermColor::Rgb {
                                r: color_bottom.r,
                                g: color_bottom.g,
                                b: color_bottom.b
                            }),
                            SetBackgroundColor(CrosstermColor::Rgb {
                                r: color_top.r,
                                g: color_top.g,
                                b: color_top.b
                            }),
                            Print("▄")
                        )?;
                    }
                }

                yummy_bits &= !(1 << mini_bit);
            }
        }
        queue!(self.stdout, EndSynchronizedUpdate)?;
        self.stdout.flush()?;
        self.dirty_rows = [0; 2];
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
