// fonts from https://fontstruct.com/fontstructions/show/1205992/8_bit_6x6_nostalgia
//    ▘  ▘   ▘
// ▛▛▌▌▛▌▌▌▌▌▌
// ▌▌▌▌▌▌▌▚▚▘▌ looks such a gud choice, but that mean i have to implement a way to add  this text
//
use crate::underterm::figlet::*;
use rael::Color;
use rael::Rael;

#[derive(Clone, Eq, PartialEq)]
pub struct StyledText {
    pub content: String,
    pub fg: Color,
}

pub fn set_figlet(
    rael: &mut Rael,
    segments: Vec<StyledText>,
    bg: Color,
    cord: (usize, usize, u8),
    is_centered: Option<(bool, bool)>,
    figlet_path: &str,
) {
    let (mut x, mut y, z) = cord;

    if !y.is_multiple_of(2) {
        return;
    }

    let font = FIGfont::from_file(figlet_path).expect("failed to load figlet font");

    let full_text: String = segments
        .iter()
        .map(|s| s.content.as_str())
        .collect::<Vec<_>>()
        .join("");

    let rendered = font
        .convert(&full_text)
        .expect("figlet conversion failed")
        .to_string();

    if let Some((cx, cy)) = is_centered {
        if cx {
            let chars_len = rendered.lines().last().unwrap_or("").chars().count();
            x = (rael.widht / 2) as usize - (chars_len / 2);
        }
        if cy {
            let lines = rendered.lines().count();
            y = (rael.height / 2) as usize - (lines / 2);
            if !y.is_multiple_of(2) {
                y += 1;
            }
        }
    }

    let mut color_per_col = Vec::new();
    let mut full_text = String::new();

    for segment in &segments {
        full_text.push_str(&segment.content);
        let char_width = font.get_char_widths(&segment.content);

        for _ in 0..char_width {
            color_per_col.push(segment.fg);
        }
    }

    let final_figure = font.convert(&full_text).expect("Final render failed");

    for (dy, line) in final_figure.to_string().lines().enumerate() {
        let y_pos = y + dy * 2;
        for (dx, (ch, &char_fg)) in line.chars().zip(color_per_col.iter()).enumerate() {
            if ch != ' ' {
                rael.set_text(x + dx, y_pos, z, bg, char_fg, ch);
            }
        }
    }
}
