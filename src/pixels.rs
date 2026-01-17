#[derive(Debug)]
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

struct Pixel {
    value: u8,
}

impl Pixel {
    pub fn new(color_index: u8) -> Self {
        Pixel { value: color_index }
    }
}

struct Screen {
    pixels: [[Pixel; 256]; 256],
}
