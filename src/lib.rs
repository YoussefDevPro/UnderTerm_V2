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
