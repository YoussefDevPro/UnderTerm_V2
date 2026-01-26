// fonts from https://fontstruct.com/fontstructions/show/1205992/8_bit_6x6_nostalgia
//    ▘  ▘   ▘
// ▛▛▌▌▛▌▌▌▌▌▌
// ▌▌▌▌▌▌▌▚▚▘▌ looks such a gud choice, but that mean i have to implement a way to add  this text
//
use figlet_rs::FIGfont;
use rael::Color;

pub fn miniwi(text: &str, color: Color, x: u16, y: u16) -> String {
    let font =
        FIGfont::from_file("./src/underterm/default.flf").expect("failed to load figlet font");

    let rendered = font
        .convert(text)
        .expect("figlet conversion failed")
        .to_string();

    let mut out = String::new();

    // Set RGB foreground color (truecolor)
    out.push_str(&format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b));

    // Write each line at the correct screen position
    for (i, line) in rendered.lines().enumerate() {
        // ANSI cursor move: row (y), column (x), both 1-based
        out.push_str(&format!("\x1b[{};{}H", y + i as u16, x));
        out.push_str(line);
    }

    // Reset ANSI state
    out.push_str("\x1b[0m");

    out
}
