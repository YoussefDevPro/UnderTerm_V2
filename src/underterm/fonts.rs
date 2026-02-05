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

// TODO: Fix flickering issue thatm ight be bc when there is a string at the end of the text

pub fn set_figlet(
    rael: &mut Rael,
    segments: &Vec<StyledText>,
    bg: Color,
    cord: (usize, usize, u8),
    is_centered: Option<(bool, bool)>,
    figlet_path: &str,
    max_widht: u16,
) {
    let (mut x, mut y, z) = cord;

    if !y.is_multiple_of(2) {
        y += 1;
    }

    let font = FIGfont::from_file(figlet_path).expect("failed to load figlet font");

    let full_text: String = segments
        .iter()
        .map(|s| s.content.as_str())
        .collect::<Vec<_>>()
        .join("");
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut rendered = Vec::new();

    for word in full_text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else {
            let space_w = font.get_char_widths(" ");
            let new_w = font.get_char_widths(&current) + space_w + font.get_char_widths(word);

            if new_w <= max_widht.into() {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = word.to_string();
            }
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    for phraze in &lines {
        rendered.push(
            font.convert(phraze)
                .expect("figlet conversion failed")
                .to_string(),
        );
    }

    if let Some((cx, cy)) = is_centered {
        if cx {
            let chars_len = rendered[0].lines().last().unwrap_or("").chars().count();
            x = (rael.widht / 2) as usize - (chars_len / 2);
        }
        if cy {
            let lines = rendered[0].lines().count();
            y = (rael.height / 2) as usize - (lines / 2);
            if !y.is_multiple_of(2) {
                y += 1;
            }
        }
    }
    let mut y_cursor = y;

    for phrase in lines {
        // ---- rebuild color_per_col for THIS phrase ----
        let mut color_per_col = Vec::new();
        let mut phrase_text = String::new();

        let mut remaining = phrase.as_str();

        for segment in segments {
            let seg = segment.content.as_str();

            if remaining.is_empty() {
                break;
            }

            if remaining.starts_with(seg) {
                phrase_text.push_str(seg);

                let w = font.get_char_widths(seg);
                for _ in 0..w {
                    color_per_col.push(segment.fg);
                }

                remaining = &remaining[seg.len()..];
            } else if remaining.starts_with(' ') && seg.starts_with(' ') {
                phrase_text.push(' ');
                color_per_col.push(segment.fg);
                remaining = &remaining[1..];
            }
        }

        let figure = font.convert(&phrase).expect("figlet conversion failed");
        let figure_str = figure.to_string();
        let figlet_lines: Vec<&str> = figure_str.lines().collect();

        for (dy, line) in figlet_lines.iter().enumerate() {
            let y_pos = y_cursor + dy * 2;
            for (dx, ch) in line.chars().enumerate() {
                let fg = color_per_col.get(dx).copied().unwrap_or(bg);

                if ch != ' ' {
                    rael.set_text(x + dx, y_pos, z, bg, fg, ch);
                }
            }
        }

        y_cursor += 8;
    }
}
