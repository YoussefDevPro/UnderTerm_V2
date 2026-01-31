use crate::underterm::fonts::StyledText;
use rael::Color;
use tokio::time::Duration;

#[derive(Clone, PartialEq, Eq)]
pub enum TextCommand {
    Text(String),
    ColoredText(String, Color),
    Delay(Duration),
}

pub enum WriterResult {
    Render(Vec<StyledText>),
    Wait(Duration),
    Finished,
}

pub struct TextWriter {
    script: Vec<TextCommand>,
    history: Vec<StyledText>,
    current_cmd_idx: usize,
    char_progress: usize,
}

impl TextWriter {
    pub fn new(script: &[TextCommand]) -> Self {
        Self {
            script: script.to_vec(),
            history: Vec::new(),
            current_cmd_idx: 0,
            char_progress: 0,
        }
    }

    pub fn next_step(&mut self) -> WriterResult {
        if self.current_cmd_idx >= self.script.len() {
            return WriterResult::Finished;
        }

        match &self.script[self.current_cmd_idx] {
            TextCommand::Text(full) | TextCommand::ColoredText(full, _) => {
                let current_color = match &self.script[self.current_cmd_idx] {
                    TextCommand::ColoredText(_, color) => *color,
                    _ => Color::new(255, 255, 255),
                };

                self.char_progress += 1;

                let mut display = self.history.clone();
                let current_part: String = full.chars().take(self.char_progress).collect();

                display.push(StyledText {
                    content: current_part,
                    fg: current_color,
                });

                if self.char_progress >= full.chars().count() {
                    self.history.push(StyledText {
                        content: full.clone(),
                        fg: current_color,
                    });
                    self.char_progress = 0;
                    self.current_cmd_idx += 1;
                }

                WriterResult::Render(display)
            }
            TextCommand::Delay(dur) => {
                let d = *dur;
                self.current_cmd_idx += 1;
                WriterResult::Wait(d)
            }
        }
    }
}
