//! Application state and main event loop.

use anyhow::Result;
use koso_core::{Config, EventBus};

/// Main application state.
pub struct App {
    config: Config,
    event_bus: EventBus,
    should_quit: bool,
}

impl App {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            config,
            event_bus: EventBus::default(),
            should_quit: false,
        })
    }

    /// Run the main event loop.
    pub async fn run(&mut self) -> Result<()> {
        // TODO: Initialize terminal
        // TODO: Create default pane with PTY
        // TODO: Start event loop
        //   - Poll crossterm events
        //   - Poll PTY output
        //   - Poll AI responses
        //   - Render UI

        tracing::info!("Koso event loop started (shell: {})", self.config.shell);

        // Placeholder — will be replaced with actual event loop
        println!("Koso 構想 v{}", env!("CARGO_PKG_VERSION"));
        println!("Shell: {}", self.config.shell);
        println!("Press Ctrl+C to exit");

        tokio::signal::ctrl_c().await?;

        Ok(())
    }
}
