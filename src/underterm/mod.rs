use rael::Color;
use std::{thread::sleep, time::Duration};

use rael::Rael;

use crate::assets;
use assets::*;

mod fonts;
use crate::underterm::fonts::set_figlet;

mod figlet;

pub enum Map {
    Intro,
    Menu,
    Exit,
}

pub async fn introduction(rael: &mut Rael) -> Map {
    let center_w = (rael.widht / 2) - 40;
    for i in ALL_IMAGES {
        //rael.set_image(*i, (center_w as usize, 0, 0));
        set_figlet(
            rael,
            "Mraow",
            Color::new(0, 0, 0),
            Color::new(255, 255, 255),
            (70, 0, 2),
            Some((true, true)),
            "src/underterm/maxiwi.flf",
        );
        let _ = rael.render().await;

        rael.clear();
        sleep(Duration::from_secs(1));
    }

    rael.clear_colors();
    let _ = rael.render().await;
    Map::Menu
}

pub async fn menu(rael: &mut Rael) -> Map {
    for i in 0..100 {
        rael.set_pixel(i, i, 2, Color::new(255, 0, 0));
    }

    let _ = rael.render().await;
    sleep(Duration::from_secs(2));

    rael.clear();
    let _ = rael.render().await;
    Map::Exit
}
