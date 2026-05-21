//! Koso — 構想
//!
//! A modern TUI terminal multiplexer with block-based output,
//! structured data rendering, and pluggable AI integration.

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

mod app;
pub mod cli;
mod ui;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting Koso 構想");

    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Load config (from --config path or default location)
    let config = koso_core::Config::load(cli.config)?
        .merge_overrides(koso_core::ConfigOverrides {
            shell: cli.shell,
            working_directory: cli.working_dir,
        });

    let mut app = app::App::new(config)?;
    app.run().await
}
