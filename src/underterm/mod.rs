use crate::underterm::fonts::StyledText;
use crate::underterm::text::*;
use crossterm::event::KeyEvent;
use crossterm::event::KeyModifiers;
use rael::Color;
use rael::ImageAsset;
use rael::Rael;
use tokio::time::sleep;
use tokio::time::Duration;

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

pub async fn check_if_we_should_exit_aah(rael: &Rael) -> bool {
    rael.inputs.snapshot().await.keys.contains(&KeyEvent::new(
        crossterm::event::KeyCode::Enter,
        KeyModifiers::NONE,
    ))
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
                TextCommand::ColoredText(
                    "MONSTERS. and mraow mrp mrp".into(),
                    Color::new(255, 255, 100),
                ),
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
                TextCommand::ColoredText("M".to_string(), Color::new(0, 0, 0)),
                TextCommand::Delay(Duration::from_secs(2)),
            ],
        },
        IntroScene {
            image: Some(INTRO_6),
            text: vec![
                TextCommand::ColoredText("M".to_string(), Color::new(0, 0, 0)),
                TextCommand::Delay(Duration::from_secs(2)),
            ],
        },
    ];
    let mut should_exit = false;
    let mut current_scene = vec![StyledText {
        content: "mraow".to_string(),
        fg: Color::new(255, 255, 255),
    }];
    for (i, scene) in intro_scenes.iter().enumerate() {
        if !should_exit {
            should_exit = check_if_we_should_exit_aah(rael).await;
        }
        let mut writer = TextWriter::new(&scene.text);
        while !should_exit {
            match writer.next_step() {
                WriterResult::Render(segments) => {
                    if !should_exit {
                        should_exit = check_if_we_should_exit_aah(rael).await;
                    }
                    current_scene = segments.clone();
                    rael.clear(); // Clear pixel/char buffers

                    if let Some(img) = scene.image {
                        rael.set_image(img, (center_w as usize, 1, 0));
                    }

                    let y = (rael.height / 2) + 27;
                    set_figlet(
                        rael,
                        &segments,
                        Color::new(0, 0, 0),
                        (0, y as usize, 0),
                        Some((true, i == 4)),
                        "./src/underterm/default.flf",
                        120,
                    );

                    let _ = rael.render(None).await;
                    tokio::select! {
                        _ = tokio::time::sleep(Duration::from_millis(40)) => {},
                        _ = async {
                            loop {
                                if rael.inputs.snapshot().await.keys.contains(&KeyEvent::new(
                                    crossterm::event::KeyCode::Enter,
                                    KeyModifiers::NONE,
                                )) {
                                    break;
                                }
                                tokio::task::yield_now().await;
                                sleep(Duration::from_millis(2)).await;
                            }
                        } => {
                            should_exit = true;
                        }
                    };
                }
                WriterResult::Wait(dur) => {
                    tokio::select! {
                        _ = tokio::time::sleep(dur) => {},
                        _ = async {
                            loop {
                                if check_if_we_should_exit_aah(rael).await {
                                    break;
                                }
                                tokio::task::yield_now().await;
                            }
                        } => {
                            should_exit = true;
                        }
                    };
                }
                WriterResult::Finished => break, // Move to next IntroScene
            }
        }
        if should_exit {
            let mut ii: f32 = 1.0;
            for _ in 0..20 {
                rael.force_clear();
                if let Some(img) = scene.image {
                    rael.set_image(img, (center_w as usize, 1, 0));
                }

                let y = (rael.height / 2) + 27;
                set_figlet(
                    rael,
                    &current_scene,
                    Color::new(0, 0, 0),
                    (0, y as usize, 0),
                    Some((true, i == 4)),
                    "./src/underterm/default.flf",
                    120,
                );

                let _ = rael.render(Some(ii)).await;
                ii -= 0.1;
                sleep(Duration::from_millis(30)).await;
            }
            rael.force_clear();
            return Map::Menu; // Exit function after fade completes
        }
        if !should_exit {
            sleep(Duration::from_secs(1)).await;
        }
    }

    rael.force_clear();
    let _ = rael.render(None).await;
    Map::Menu
}

pub async fn menu(rael: &mut Rael) -> Map {
    for i in 0..rael.height as usize {
        rael.set_pixel(i, i, 2, Color::new(255, 0, 0));
        rael.set_pixel(i, rael.height as usize - i, 2, Color::new(255, 0, 0));
    }

    let _ = rael.render(None).await;
    sleep(Duration::from_secs(2)).await;

    rael.clear();
    let _ = rael.render(None).await;
    Map::Exit
}
