mod rael;

use crate::rael::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut rael = Rael::new();
    rael.setup_events();

    let recv = rael.enable_mouse();
    println!("rentering the loop");

    loop {
        let _ = rael.update_wsize();

        let mouse_event = recv.try_recv().unwrap();

        match mouse_event {
            MouseEvent::Move { x, y } => {
                rael.screen.set_pixel(x, y, 1, Color::new(255, 0, 0));
            }
            _ => { /* do nothing :p */ }
        }

        rael.render();
        rael.clear();

        sleep(Duration::from_millis(16));
    }
}
