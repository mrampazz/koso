//! Shared types used across Koso crates.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// A command entered by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    /// Raw command string.
    pub raw: String,

    /// Working directory when the command was executed.
    pub cwd: String,

    /// Timestamp.
    pub timestamp: SystemTime,

    /// Exit code (None if still running).
    pub exit_code: Option<i32>,
}

/// A block represents one command + its output as a discrete unit.
#[derive(Debug, Clone)]
pub struct Block {
    /// Unique block ID.
    pub id: u64,

    /// The command that produced this block.
    pub command: Command,

    /// Output data (stdout + stderr interleaved).
    pub output: Vec<u8>,

    /// Detected output kind.
    pub kind: BlockKind,

    /// Whether the block is collapsed in the UI.
    pub collapsed: bool,
}

/// The detected kind of output in a block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockKind {
    /// Plain text output.
    Plain,

    /// JSON data (can be rendered as a tree).
    Json,

    /// Tabular data (can be rendered as a sortable table).
    Table,

    /// Log output (can be filtered by level).
    Log,

    /// Error output.
    Error,

    /// Empty (command produced no output).
    Empty,
}
