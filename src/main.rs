mod underterm;

use rael::*;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let mut rael = Rael::new(std::io::stdout(), "Rael Simple Example")?;

    Ok(())
}
