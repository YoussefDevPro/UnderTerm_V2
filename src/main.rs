use std::{thread::sleep, time::Duration};

use rael::Rael;

fn main() {
    let mut RAEL = Rael::new();
    RAEL.setup_events();
    loop {
        let _ = RAEL.update_size();
        println!("{}x{}", RAEL.widht, RAEL.height);
        sleep(Duration::from_secs(2));
    }
}
