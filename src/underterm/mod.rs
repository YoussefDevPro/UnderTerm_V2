use crate::underterm::text::*;
use rael::Color;
use rael::ImageAsset;
use rael::Rael;
use std::{thread::sleep, time::Duration};

use crate::assets;
use crate::underterm::text::TextCommand;
use assets::*;

mod fonts;
use crate::underterm::fonts::set_figlet;

mod figlet;
mod text;

pub enum Map {
    Intro,
    Menu,
    Exit,
}

struct IntroScene {
    text: Vec<TextCommand>,
    image: Option<ImageAsset<120, 66>>,
}

pub async fn introduction(rael: &mut Rael) -> Map {
    let center_w = (rael.widht / 2) - 60;
    let intro_scenes: [IntroScene; 10] = [
        IntroScene {
            image: Some(INTRO_1),
            text: vec![
                TextCommand::Text("Long ago, two races ruled over Earth: ".into()),
                TextCommand::ColoredText("HUMANS ".into(), Color::new(255, 255, 100)),
                TextCommand::Text("and ".into()),
                TextCommand::ColoredText("MONSTERS.".into(), Color::new(255, 255, 100)),
            ],
        },
        IntroScene {
            image: Some(INTRO_2),
            text: vec![TextCommand::Text(
                "One day, war broke out between the two races.".into(),
            )],
        },
        IntroScene {
            image: Some(INTRO_3),
            text: vec![TextCommand::Text(
                "After a long battle, the humans were victorious.".into(),
            )],
        },
        IntroScene {
            image: Some(INTRO_3),
            text: vec![TextCommand::Text(
                "They sealed the monsters underground with a magic spell.".into(),
            )],
        },
        IntroScene {
            image: None,
            text: vec![
                TextCommand::Delay(Duration::from_millis(500)),
                TextCommand::Text("Many years later...".into()),
            ],
        },
        IntroScene {
            image: Some(INTRO_4),
            text: vec![TextCommand::ColoredText(
                "MT. EBOTT".into(),
                Color::new(255, 255, 0),
            )],
        },
        IntroScene {
            image: Some(INTRO_4),
            text: vec![TextCommand::Text("201X".into())],
        },
        IntroScene {
            image: Some(INTRO_4),
            text: vec![TextCommand::Text(
                "Legends say that those who climb the mountain never return.".into(),
            )],
        },
        IntroScene {
            image: Some(INTRO_5),
            text: vec![
                TextCommand::Text(" ".to_string()),
                TextCommand::Delay(Duration::from_secs(2)),
            ],
        },
        IntroScene {
            image: Some(INTRO_6),
            text: vec![
                TextCommand::Text(" ".to_string()),
                TextCommand::Delay(Duration::from_secs(2)),
            ],
        },
    ];
    for (i, scene) in intro_scenes.iter().enumerate() {
        let mut writer = TextWriter::new(&scene.text);
        loop {
            match writer.next_step() {
                WriterResult::Render(segments) => {
                    rael.clear(); // Clear pixel/char buffers

                    if let Some(img) = scene.image {
                        rael.set_image(img, (center_w as usize, 0, 0));
                    }

                    let y = (rael.height / 2) + 27;
                    set_figlet(
                        rael,
                        segments,
                        Color::new(0, 0, 0),
                        (0, y as usize, 0),
                        Some((true, i == 4)),
                        "./src/underterm/default.flf",
                    );

                    let _ = rael.render().await;
                    sleep(Duration::from_millis(40));
                }
                WriterResult::Wait(dur) => {
                    sleep(dur);
                }
                WriterResult::Finished => break, // Move to next IntroScene
            }
        }

        sleep(Duration::from_secs(1));
    }

    rael.force_clear();
    let _ = rael.render().await;
    Map::Menu
}

pub async fn menu(rael: &mut Rael) -> Map {
    for i in 0..rael.height as usize {
        rael.set_pixel(i, i, 2, Color::new(255, 0, 0));
        rael.set_pixel(i, rael.height as usize - i, 2, Color::new(255, 0, 0));
    }

    let _ = rael.render().await;
    sleep(Duration::from_secs(2));

    rael.clear();
    let _ = rael.render().await;
    Map::Exit
}
