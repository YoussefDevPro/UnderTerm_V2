use crossterm::event::{Event, KeyEvent, MouseEvent};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Snapshot of the current input state.
///
/// Contains the mouse, keys, focus state, and terminal resize info.
/// Useful for reading terminal events at a single point in time.
#[derive(Default, Clone)]
pub struct InputSnapshot {
    /// Last mouse event, if any
    pub mouse: Option<MouseEvent>,
    /// Keys pressed since last snapshot
    pub keys: Vec<KeyEvent>,
    /// Whether the terminal lost focus
    pub focus_lost: bool,
    /// Terminal size change (columns, rows) if resized
    pub resize: Option<(u16, u16)>,
}

/// Async terminal input handler.
///
/// Handles keyboard, mouse, focus, and resize events in the background.
/// Use `snapshot()` to read current input, or `take_snapshot()` to read and reset events.
#[derive(Clone)]
pub struct Input {
    state: Arc<Mutex<InputSnapshot>>,
}

impl Input {
    /// Create a new input handler from a stream of terminal events.
    ///
    /// # Parameters
    /// - `events`: A futures stream of [`Event`] objects
    ///
    /// # Example
    /// ```rust
    /// let input = Input::new(EventStream::new());
    /// ```
    pub fn new(
        mut events: impl futures::Stream<Item = std::io::Result<Event>>
            + Send
            + 'static
            + std::marker::Unpin,
    ) -> Self {
        let state = Arc::new(Mutex::new(InputSnapshot::default()));
        let state_bg = state.clone();

        // Spawn a background task to continuously update the input state
        tokio::spawn(async move {
            while let Some(Ok(event)) = events.next().await {
                let mut s = state_bg.lock().await;
                match event {
                    Event::Mouse(mouse) => s.mouse = Some(mouse),
                    Event::Key(key) => s.keys.push(key),
                    Event::Resize(width, height) => s.resize = Some((width, height)),
                    Event::FocusLost => s.focus_lost = true,
                    Event::FocusGained => s.focus_lost = false,
                    _ => {}
                }
            }
        });

        Self { state }
    }

    /// Get a snapshot of the current input state.
    ///
    /// This does **not reset** keys, mouse, or resize info.
    /// Useful if you just want to inspect the current state without clearing events.
    pub async fn snapshot(&self) -> InputSnapshot {
        self.state.lock().await.clone()
    }

    /// Take a snapshot of the current input and **reset** events.
    ///
    /// After calling this, the keys, mouse, and resize events are cleared.
    /// Useful for per-frame input handling in a game loop.
    ///
    /// # Example
    /// ```rust
    /// let snap = input.take_snapshot().await;
    /// if let Some(mouse) = snap.mouse {
    ///     println!("Mouse event: {:?}", mouse);
    /// }
    /// for key in snap.keys {
    ///     println!("Key pressed: {:?}", key);
    /// }
    /// ```
    pub async fn take_snapshot(&self) -> InputSnapshot {
        let mut s = self.state.lock().await;
        let snap = s.clone();

        s.mouse = None;
        s.keys.clear();
        s.resize = None;

        snap
    }
}
