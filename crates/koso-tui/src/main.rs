//! Koso — 構想
//!
//! A modern TUI terminal multiplexer with block-based output,
//! structured data rendering, and pluggable AI integration.

use anyhow::Result;
use tracing_subscriber::EnvFilter;

mod app;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Koso 構想");

    let config = koso_core::Config::load()?;
    let mut app = app::App::new(config)?;
    app.run().await
}
