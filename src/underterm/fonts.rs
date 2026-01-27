// fonts from https://fontstruct.com/fontstructions/show/1205992/8_bit_6x6_nostalgia
//    ▘  ▘   ▘
// ▛▛▌▌▛▌▌▌▌▌▌
// ▌▌▌▌▌▌▌▚▚▘▌ looks such a gud choice, but that mean i have to implement a way to add  this text
//
use figlet_rs::FIGfont;
use rael::Color;
use rael::Rael;

pub fn miniwi(rael: &mut Rael, text: &str, color: Color, x: usize, y: usize, z: u8) {
    let font =
        FIGfont::from_file("./src/underterm/default.flf").expect("failed to load figlet font");

    let rendered = font
        .convert(text)
        .expect("figlet conversion failed")
        .to_string();

    for (dy, line) in rendered.lines().enumerate() {
        for (dx, ch) in line.chars().enumerate() {
            // Skip empty space to avoid overwriting background
            if ch == ' ' || dy % 2 != 0 {
                continue;
            }

            rael.set_pixel(x + dx, y + dy, z, color, Some(ch));
        }
    }
}
