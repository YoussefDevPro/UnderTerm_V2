// fonts from https://fontstruct.com/fontstructions/show/1205992/8_bit_6x6_nostalgia
//    ▘  ▘   ▘
// ▛▛▌▌▛▌▌▌▌▌▌
// ▌▌▌▌▌▌▌▚▚▘▌ looks such a gud choice, but that mean i have to implement a way to add  this text
//
use crate::underterm::figlet::*;
use rael::Color;
use rael::Rael;

pub fn miniwi(rael: &mut Rael, text: &str, bg: Color, fg: Color, x: usize, y: usize, z: u8) {
    if !y.is_multiple_of(2) {
        return;
    }
    let font =
        FIGfont::from_file("./src/underterm/default.flf").expect("failed to load figlet font");

    let rendered = font
        .convert(text)
        .expect("figlet conversion failed")
        .to_string();

    for (dy, line) in rendered.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();

        for dx in 0..chars.len() {
            let ch = chars[dx];
            if ch == ' ' {
                continue;
            }
            rael.set_text(x + dx, y + dy * 2, z, bg, fg, ch);
        }
    }
}
