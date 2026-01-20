mod rael;

use crate::rael::*;

use std::{thread::sleep, time::Duration};

fn main() {
    let rael = Rael::new();
    rael.setup_events();

    let _ = rael.enable_mouse();
}
