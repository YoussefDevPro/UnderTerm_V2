mod assets;
mod underterm;

use rael::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// 27 fps WTF
// primegeon got like 1600fps WTF

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut rael = Rael::new(std::io::stdout(), "Rael Pixel Stress + FPS")?;

    let mut rng = SmallRng::from_rng(&mut rand::rng());

    let mut frames: u64 = 0;
    let mut last_fps = Instant::now();
    let mut fps: u64 = 0;

    loop {
        frames += 1;

        let w = rael.widht as usize;
        let h = rael.height as usize;

        for _ in 0..(w * h) {
            let x = rng.random_range(0..w);
            let y = rng.random_range(0..h);

            rael.set_pixel(
                x,
                y,
                rng.random(), // z
                Color::new(rng.random::<u8>(), rng.random::<u8>(), rng.random::<u8>()), // bg
            );
        }

        if last_fps.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            last_fps = Instant::now();
        }

        let fps_str = format!("FPS: {}", fps);

        for (i, ch) in fps_str.chars().enumerate() {
            rael.set_text(
                i,
                0,
                255,                       // always on top
                Color::new(0, 0, 0),       // bg
                Color::new(255, 255, 255), // fg
                ch,
            );
        }

        rael.render().await?;

        // Yield to the scheduler (and your CPU fan)
        sleep(Duration::from_millis(1)).await;
    }
}
