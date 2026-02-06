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

pub struct Scenario {
    bg: Color,
    x: usize,
    y: usize,
    z: u8,
    is_centered_x: bool,
    is_centered_y: bool,
    figlet: FIGfont,
    max_widht: u16,
}

impl Scenario {
    pub fn new(
        bg: Color,
        cord: (usize, usize, u8),
        is_centered: Option<(bool, bool)>,
        figlet_path: &str,
        max_widht: u16,
    ) -> Self {
        let (x, mut y, z) = cord;
        if !y.is_multiple_of(2) {
            y += 1;
        }
        let font = FIGfont::from_file(figlet_path).expect("failed to load figlet font");
        Self {
            bg,
            x,
            y,
            z,
            is_centered_x: is_centered.unwrap_or((false, false)).0,
            is_centered_y: is_centered.unwrap_or((false, false)).1,
            figlet: font,
            max_widht,
        }
    }

    pub fn set_text(&mut self, rael: &mut Rael, segments: &Vec<StyledText>) {
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
            } else if word == "|" {
                lines.push(current);
                current = String::new();
            } else {
                let new_w =
                    self.figlet.get_char_widths(&current) + 1 + self.figlet.get_char_widths(word);

                if new_w <= self.max_widht.into() {
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
                self.figlet
                    .convert(phraze)
                    .expect("figlet conversion failed")
                    .to_string(),
            );
        }

        if self.is_centered_x {
            let chars_len = rendered[0].lines().last().unwrap_or("").chars().count();
            self.x = (rael.widht / 2) as usize - (chars_len / 2);
        }
        if self.is_centered_y {
            let lines: usize = rendered.iter().map(|b| b.lines().count()).sum();
            self.y = (rael.height / 2) as usize - (lines / 2);
            if !self.y.is_multiple_of(2) {
                self.y += 1;
            }
        }
        let mut y_cursor = self.y;
        let mut global_char_index = 0;

        for (ii, phrase) in lines.iter().enumerate() {
            let mut color_per_col = Vec::new();
            if self.is_centered_x {
                let chars_len = rendered[ii].lines().last().unwrap_or("").chars().count();
                self.x = (rael.widht / 2) as usize - (chars_len / 2);
            }
            for ch in phrase.chars() {
                let mut current_pos = 0;
                let mut found_color = self.bg;

                for segment in segments {
                    let seg_len = segment.content.chars().count();

                    if global_char_index < current_pos + seg_len {
                        found_color = segment.fg;
                        break;
                    }
                    current_pos += seg_len;
                }

                let char_w = self.figlet.get_char_widths(&ch.to_string());
                for _ in 0..char_w {
                    color_per_col.push(found_color);
                }

                global_char_index += 1;
            }
            global_char_index += 1;

            let figure = self
                .figlet
                .convert(&phrase)
                .expect("figlet conversion failed");
            let figure_str = figure.to_string();
            let figlet_lines: Vec<&str> = figure_str.lines().collect();

            for (dy, line) in figlet_lines.iter().enumerate() {
                let y_pos = y_cursor + dy * 2;
                for (dx, ch) in line.chars().enumerate() {
                    let fg = color_per_col.get(dx).copied().unwrap_or(self.bg);

                    if ch != ' ' {
                        rael.set_text(self.x + dx, y_pos, self.z, self.bg, fg, ch);
                    }
                }
            }

            y_cursor += 8;
        }
    }
}
