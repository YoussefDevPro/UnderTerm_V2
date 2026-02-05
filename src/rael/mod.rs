use bimap::BiMap;
use crossterm::cursor;
use crossterm::event::{
    DisableFocusChange, DisableMouseCapture, EnableMouseCapture, EventStream,
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
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
use std::io::{self, Stdout, Write};

use crate::rael::input::Input;

mod input;

const MAX: usize = 512;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn make_it_more_deltarune(&self, deltarune: f32) -> Self {
        let deltarune = deltarune.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 * deltarune) as u8,
            g: (self.g as f32 * deltarune) as u8,
            b: (self.b as f32 * deltarune) as u8,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageAsset<const W: usize, const H: usize> {
    pub pixels: [[u16; W]; H],
    pub colors: &'static [Color],
}

pub struct Rael {
    pub widht: u16,
    pub height: u16,
    pub pixels: [[u16; MAX]; MAX],
    pub z_buffer: [[u8; MAX]; MAX],
    pub colors: BiMap<u16, Color>,
    pub stdout: Stdout,
    pub old: Box<[[u16; MAX]; MAX]>,
    pub inputs: Input,
    pub chars: [[char; MAX]; MAX / 2],
    pub old_chars: Box<[[char; MAX]; MAX / 2]>,
    pub dirty_rows: [u128; 2],
}

impl Rael {
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
        let mut colors = BiMap::new();
        colors.insert(0, Color::new(0, 0, 0));

        Ok(Rael {
            widht: win.columns,
            height: win.rows * 2,
            pixels: [[0; MAX]; MAX],
            z_buffer: [[0; MAX]; MAX],
            colors,
            stdout,
            old: Box::new([[1; MAX]; MAX]),
            inputs: Input::new(reader),
            chars: [[' '; MAX]; MAX / 2],
            dirty_rows: [0; 2],
            old_chars: Box::new([[' '; MAX]; MAX / 2]),
        })
    }

    fn get_or_insert_color(&mut self, color: Color) -> u16 {
        //*self.colors.entry(color).or_insert_with(|| {
        //    let new_index = self.colors.len() as u16;
        //    self.colors.push(color);
        //    new_index
        //})
        if self.colors.contains_right(&color) {
            *self.colors.get_by_right(&color).unwrap()
        } else {
            let new_index = self.colors.len() as u16;
            self.colors.insert(new_index, color);
            new_index
        }
    }

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
        if x > MAX || y > MAX / 2 {
            panic!("y={y} and x={x}, one of them exceeds MAX:{MAX}");
        }
        let y = if y.is_multiple_of(2) { y } else { y - 1 };
        if self.z_buffer[y][x] <= z {
            self.pixels[y][x] = self.get_or_insert_color(bg);
            self.z_buffer[y][x] = z;
            self.chars[y / 2][x] = cchar;
            self.pixels[y + 1][x] = self.get_or_insert_color(fg);
            self.dirty_rows[(y / 2) / 128] |= 1 << ((y / 2) % 128);
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

    pub fn clear(&mut self) {
        *self.old = self.pixels;
        *self.old_chars = self.chars;
        self.dirty_rows = [0; 2];
        for y in 0..self.height as usize {
            let pixel_y1 = y * 2;
            let pixel_y2 = y * 2 + 1;

            for x in 0..self.widht as usize {
                let char_dirty = self.chars[y][x] != ' ';
                let pixel_dirty = self.pixels[pixel_y1][x] != 0
                    || (pixel_y2 < MAX && self.pixels[pixel_y2][x] != 0);

                if char_dirty || pixel_dirty {
                    self.dirty_rows[y / 128] |= 1u128 << (y % 128);
                    break; // This terminal row is dirty; skip to next y
                }
            }
        }
        self.z_buffer = [[0; MAX]; MAX];
        self.pixels = [[0; MAX]; MAX];
        self.chars = [[' '; MAX]; MAX / 2];
    }

    pub fn clear_colors(&mut self) {
        let mut colors = BiMap::new();
        colors.insert(0, Color::new(0, 0, 0));
        self.colors = colors;
    }

    pub fn force_clear(&mut self) {
        // all those clears was not enough, so i made this >:3
        *self.old = [[255; MAX]; MAX];
        self.pixels = [[0; MAX]; MAX];
        self.z_buffer = [[0; MAX]; MAX];
        self.chars = [[' '; MAX]; MAX / 2];
        self.dirty_rows = [u128::MAX; 2];
        self.clear_colors();
    }

    pub async fn render(&mut self, deltarune: Option<f32>) -> io::Result<()> {
        let deltarune = deltarune.unwrap_or(1.0);
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

                if render_y < self.height.into() {
                    queue!(self.stdout, cursor::MoveTo(0, y as u16))?;
                    let row_top = self.pixels[render_y];
                    let row_bottom = self.pixels[render_y + 1];
                    let old_row_top = self.old[render_y];
                    let old_row_bottom = self.old[render_y + 1];
                    let chars = self.chars[y];
                    let old_chars = self.old_chars[y];

                    //let mut skip_count = 0;

                    for x in 0..self.widht as usize {
                        let top = row_top[x];
                        let bottom = row_bottom[x];

                        let old_top = old_row_top[x];
                        let old_bottom = old_row_bottom[x];

                        let (new_char, old_char) = (chars[x], old_chars[x]);
                        if top == old_top && bottom == old_bottom && new_char == old_char {
                            queue!(self.stdout, cursor::MoveRight(1))?;
                            continue;
                        }

                        //if skip_count > 0 {
                        //    queue!(self.stdout, cursor::MoveTo(0, skip_count))?;
                        //}

                        let color_top = self
                            .colors
                            .get_by_left(&top)
                            .unwrap()
                            .make_it_more_deltarune(deltarune);
                        let color_bottom = self
                            .colors
                            .get_by_left(&bottom)
                            .unwrap()
                            .make_it_more_deltarune(deltarune);
                        if new_char != ' ' {
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
                                Print(new_char)
                            )?;
                        } else if color_top == color_bottom {
                            queue!(
                                self.stdout,
                                SetBackgroundColor(CrosstermColor::Rgb {
                                    r: color_top.r,
                                    g: color_top.g,
                                    b: color_top.b
                                }),
                                Print(" ")
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
    fn drop(&mut self) {
        let _ = execute!(
            self.stdout,
            DisableMouseCapture,
            DisableFocusChange,
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
