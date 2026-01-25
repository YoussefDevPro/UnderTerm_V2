mod assets;
mod underterm;

use rael::*;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut rael = Rael::new(std::io::stdout(), "Rael Simple Example")?;

    rael.set_image(
        *assets::INTRO_1_PIXELS,
        *assets::INTRO_1_COLORS,
        *assets::INTRO_1_WIDTH,
        *assets::INTRO_1_HEIGHT,
        (0, 0, 0),
    );
    let _ = rael.render().await;
    let _ = sleep(Duration::from_secs(3)).await;

    Ok(())
}
