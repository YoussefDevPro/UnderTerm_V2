#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    // 5 red | 6 green | 5 blue
    pub value: u16,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            value: u16::from(r >> 3) << 11 | u16::from(g >> 2) << 5 | u16::from(b >> 3),
        }
    }

    pub fn r(&self) -> u8 {
        let n: u8 = (self.value >> 11).try_into().unwrap();
        (n << 3) | (n >> 2)
    }

    pub fn g(&self) -> u8 {
        let n: u8 = ((self.value >> 5) & 0x3f).try_into().unwrap();
        (n << 2) | (n >> 4)
    }

    pub fn b(&self) -> u8 {
        let n: u8 = (self.value & 0x1f).try_into().unwrap();
        (n << 3) | (n >> 2)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub value: u8,
}

impl Pixel {
    pub fn new(color_index: u8) -> Self {
        Pixel { value: color_index }
    }
}

#[derive(Debug, Clone)]
pub struct Screen {
    pub pixels: [[Pixel; 256]; 256],
    pub colors: Vec<Color>,
    pub z_buffer: [[u8; 128]; 256],
}

#[derive(Debug, Clone, Copy)]
pub struct Old_Screen {
    pub pixels: [[Pixel; 256]; 256],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            pixels: [[Pixel::new(0); 256]; 256],
            colors: vec![Color::new(0, 0, 0)],
            z_buffer: [[0; 128]; 256],
        }
    }
    // asked da ai to do it bc im lazy
    fn get_or_insert_color(&mut self, color: Color) -> u8 {
        if let Some(i) = self.colors.iter().position(|&mraow| mraow == color) {
            i as u8
        } else {
            let i = self.colors.len();
            self.colors.push(color);
            i as u8
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, mut z: u8, color: Color) {
        if z >= 16 {
            z = 15;
        };
        let z_index = self.z_buffer[y as usize][usize::from(x) >> 1];
        let current_z: u8 = if x % 2 == 1 {
            z_index & 0xf
        } else {
            z_index >> 4
        };
        if current_z > z {
            // do nothing, the pixel is already on the bottom of the other pixel, that mean we cant
            // see it
        } else {
            self.pixels[y as usize][x as usize].value = self.get_or_insert_color(color);
            self.z_buffer[y as usize][usize::from(x) >> 1] = z;
        }
    }

    pub(crate) fn clear(&mut self) -> Option<Old_Screen> {
        let old = Old_Screen {
            pixels: self.pixels,
        };
        self.pixels = [[Pixel::new(0); 256]; 256];
        self.z_buffer = [[0; 128]; 256];
        Some(old)
    }

    pub(crate) fn render(&self, widht: u16, height: u16, old: Option<Old_Screen>) -> String {
        let mut buffer = String::new();
        for y in (0..height).step_by(2) {
            for x in 0..widht {
                buffer.push_str(&format!("\u{1b}[{};{}H", (y + 1).div_ceil(2), x + 1));
                let top = self.pixels[y as usize][x as usize].value;
                let bottom = if y + 1 < height {
                    self.pixels[(y + 1) as usize][x as usize].value
                } else {
                    top
                };
                if !old.is_none() {
                    let old_top = old.unwrap().pixels[y as usize][x as usize].value;
                    let old_bottom = if y + 1 < height {
                        old.unwrap().pixels[(y + 1) as usize][x as usize].value
                    } else {
                        old_top
                    };
                    if top == old_top && bottom == old_bottom {
                        continue;
                    }
                }
                let top = self.colors[top as usize];
                let bottom = self.colors[bottom as usize];
                if top.value == bottom.value {
                    buffer.push_str(&format!(
                        "\u{1b}[38;2;{};{};{}m█",
                        top.r(),
                        top.g(),
                        top.b()
                    ));
                } else {
                    buffer.push_str(&format!(
                        "\u{1b}[48;2;{};{};{}m\u{1b}[38;2;{};{};{}m▄",
                        top.r(),
                        top.g(),
                        top.b(),
                        bottom.r(),
                        bottom.g(),
                        bottom.b()
                    ));
                }
            }
        }
        buffer
    }
}
