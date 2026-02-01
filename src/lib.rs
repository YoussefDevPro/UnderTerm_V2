//! Rael is a "game engine" running in the terminal, well, technically its just a set_pixel
//! function but yknow, i let u guys do the rest...
//! it uses crossterm for the terminal stuff, here is a simple example to draw an X in the terminal
//! # Example
//! ```rust
//! mod rael;
//!
//! use rael::{Color, Rael};
//! use tokio::time::{sleep, Duration};
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> std::io::Result<()> {
//!     let mut rael = Rael::new(std::io::stdout(), "Rael Simple Example")?;
//!
//!     for i in 0..10 {
//!         rael.set_pixel(i, i, 1, Color::new(255, 0, 0));
//!         rael.set_pixel(i, 9 - i, 1, Color::new(0, 255, 0));
//!     }
//!
//!     rael.render().await?;
//!
//!     sleep(Duration::from_secs(2)).await;
//!
//!     rael.clear();
//!     rael.render().await?;
//!
//!     Ok(())
//! }
//! ```
//! the "engine" uses kitties keyboard protocol, and all of the events can be triggered using
//! `rael.inputs.take_snapshot()` to take the current input

mod rael;
pub use rael::*;

use rand::Rng;
use std::io;
use std::time::{Duration, Instant};

pub async fn run_stress_test(rael: &mut Rael) -> io::Result<()> {
    let mut rng = rand::rng();

    // Define a fixed palette to pick from
    let palette = [
        Color::new(255, 0, 0),     // Red
        Color::new(0, 255, 0),     // Green
        Color::new(0, 0, 255),     // Blue
        Color::new(255, 255, 0),   // Yellow
        Color::new(255, 0, 255),   // Magenta
        Color::new(0, 255, 255),   // Cyan
        Color::new(255, 255, 255), // White
    ];

    let mut frame_count = 0;
    let mut last_check = Instant::now();
    let mut fps_display = String::from("FPS: 0.00");

    loop {
        let num_pixels = rng.random_range(1..1000);

        for _ in 0..num_pixels {
            let x = rng.random_range(0..rael.widht as usize);
            let y = rng.random_range(0..rael.height as usize);
            let color = palette[rng.random_range(0..palette.len())];
            rael.set_pixel(x, y, 0, color);
        }

        frame_count += 1;
        let elapsed = last_check.elapsed();
        if elapsed >= Duration::from_secs(1) {
            let fps = frame_count as f64 / elapsed.as_secs_f64();
            fps_display = format!("FPS: {:.2}", fps);
            frame_count = 0;
            last_check = Instant::now();
        }

        for (i, c) in fps_display.chars().enumerate() {
            rael.set_text(i, 0, 255, Color::new(0, 0, 0), Color::new(255, 255, 255), c);
        }

        rael.render().await?;
        rael.clear();
    }
}
