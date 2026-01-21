mod rael;

use std::{thread::sleep, time::Duration};

use crate::rael::*;

fn main() {
    let mut rael = Rael::new();
    rael.setup_events();
    let recv = rael.enable_mouse();
    rael.render();
    loop {
        let _ = rael.update_wsize();
        let mut last_move = None;
        while let Ok(ev) = recv.try_recv() {
            if let MouseEvent::Move { x, y } = ev {
                last_move = Some((x, y));
            }
        }
        if let Some((x, y)) = last_move {
            rael.screen.set_pixel(x, y * 2, 15, Color::new(255, 0, 0));
            rael.render();
            rael.clear();
        }
    }
}
