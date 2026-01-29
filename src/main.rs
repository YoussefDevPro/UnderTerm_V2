mod assets;
mod underterm;

use crate::underterm::Map;
use rael::*;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut rael = Rael::new(std::io::stdout(), "Rael Simple Example")?;
    let mut current_map = Map::Intro;
    rael.force_clear();
    let _ = rael.render().await;

    loop {
        current_map = match current_map {
            Map::Intro => underterm::introduction(&mut rael).await,
            Map::Menu => underterm::menu(&mut rael).await,
            Map::Exit => break,
        }
    }
    Ok(())
}
