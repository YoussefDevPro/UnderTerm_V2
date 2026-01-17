use rael::Rael;
use std::{thread::sleep, time::Duration};

use crate::pixels::Color;
mod pixels;

fn main() {
    //let mut rael = Rael::new();
    //rael.setup_events();
    //loop {
    //    let _ = rael.update_wsize();
    //    println!("{}x{}", rael.widht, rael.height);
    //    sleep(Duration::from_secs(1));
    //}
    let color = Color::new(255, 245, 200);
    dbg!(&color);
    dbg!(&color.r());
    dbg!(&color.g());
    dbg!(&color.b());
}
