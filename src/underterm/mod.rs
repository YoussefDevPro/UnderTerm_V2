use rael::Color;
use std::{thread::sleep, time::Duration};

use rael::Rael;

use crate::assets;
use assets::*;

mod fonts;
use crate::underterm::fonts::miniwi;

pub enum Map {
    Intro,
    Menu,
    Exit,
}

pub async fn introduction(rael: &mut Rael) -> Map {
    let center_w = (rael.widht / 2) - 40;
    for i in ALL_IMAGES {
        rael.set_image(*i, (center_w as usize, 0, 0));
        let _ = rael.render().await;

        rael.clear();
        sleep(Duration::from_secs(1));
    }
    rael.clear_colors();
    Map::Menu
}

pub async fn menu(rael: &mut Rael) -> Map {
    for i in 0..10 {
        rael.set_pixel(i, i, 2, Color::new(255, 0, 0), Some('O'));
        rael.set_pixel(i, 9 - i, 1, Color::new(0, 255, 0), None);
    }

    let _ = rael.render().await;
    //let _ = rael.render_custom(miniwi(
    //    "UNDERTERM",
    //    Color::new(255, 255, 255),
    //    rael.widht / 2 - 30,
    //    40,
    //));

    sleep(Duration::from_secs(2));

    rael.clear();
    let _ = rael.render().await;
    Map::Exit
}
