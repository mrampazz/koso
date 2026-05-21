//! Event bus for inter-component communication.

use tokio::sync::broadcast;

/// Events that flow through the Koso system.
#[derive(Debug, Clone)]
pub enum Event {
    /// A command was submitted by the user.
    CommandSubmitted { command: String, pane_id: u32 },

    /// A command produced output.
    CommandOutput { pane_id: u32, data: Vec<u8> },

    /// A command finished executing.
    CommandFinished { pane_id: u32, exit_code: i32 },

    /// User requested AI assistance.
    AiRequest { prompt: String, context: Option<String> },

    /// AI produced a response.
    AiResponse { content: String },

    /// Pane was created / destroyed / resized.
    PaneCreated { pane_id: u32 },
    PaneDestroyed { pane_id: u32 },
    PaneResized { pane_id: u32, cols: u16, rows: u16 },

    /// User input (keyboard / mouse).
    KeyPress(crossterm::event::KeyEvent),

    /// Application should quit.
    Quit,
}

/// Broadcast-based event bus.
#[derive(Debug, Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn send(&self, event: Event) -> Result<(), broadcast::error::SendError<Event>> {
        self.sender.send(event).map(|_| ())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}
