use std::path::PathBuf;

use clap::Parser;

/// 構想 — A modern TUI terminal multiplexer
#[derive(Debug, Parser)]
#[command(name = "koso", version)]
pub struct Cli {
    /// Shell to use (e.g., /bin/zsh)
    #[arg(short, long)]
    pub shell: Option<String>,

    /// Starting working directory
    #[arg(short, long)]
    pub working_dir: Option<PathBuf>,

    /// Path to config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
