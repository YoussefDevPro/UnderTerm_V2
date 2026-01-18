mod lib;

use crate::lib::*;

use std::{thread::sleep, time::Duration};

fn main() {
    let mut rael = Rael::new();
    rael.setup_events();

    let mut x = 0;
    let mut y = 0;
    let mut dx: i32 = 1;
    let mut dy: i32 = 1;

    loop {
        let _ = rael.update_wsize();

        x += dx;
        y += dy;

        if x == 0 || x + 1 >= rael.widht.into() {
            dx = -dx;
        }
        if y == 0 || y + 1 >= rael.height.into() {
            dy = -dy;
        }

        rael.screen.set_pixel(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            1,
            Color::new(255, 0, 0),
        );

        rael.render();
        rael.clear();

        sleep(Duration::from_millis(16));
    }
}
