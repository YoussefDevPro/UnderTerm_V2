use std::usize;

// fonts from https://fontstruct.com/fontstructions/show/1205992/8_bit_6x6_nostalgia
//    ▘  ▘   ▘
// ▛▛▌▌▛▌▌▌▌▌▌
// ▌▌▌▌▌▌▌▚▚▘▌ looks such a gud choice, but that mean i have to implement a way to add  this text
//
use crate::underterm::figlet::*;
use rael::Color;
use rael::Rael;

pub fn set_figlet(
    rael: &mut Rael,
    text: &str,
    bg: Color,
    fg: Color,
    cord: (usize, usize, u8), // x y z
    is_centered: Option<(bool, bool)>,
    figlet_path: &str,
) {
    let mut y = cord.0;
    let mut x = cord.1;
    let z = cord.2;

    if !y.is_multiple_of(2) {
        return;
    }

    let font = FIGfont::from_file(figlet_path).expect("failed to load figlet font");

    let rendered = font
        .convert(text)
        .expect("figlet conversion failed")
        .to_string();

    if let Some((cx, cy)) = is_centered {
        if cx {
            x = (rael.widht / 2) as usize - (rendered.lines().last().unwrap().len() / 2);
        }
        if cy {
            y = (rael.height / 2) as usize - (rendered.lines().count() / 2);
            if !y.is_multiple_of(2) {
                y += 1
            }
        }
    }

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
