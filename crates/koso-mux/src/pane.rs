//! Pane layout management.
//!
//! Handles splitting, resizing, and focus management across panes.

use serde::{Deserialize, Serialize};

/// A single pane in the terminal multiplexer.
#[derive(Debug)]
pub struct Pane {
    pub id: u32,
    pub title: Option<String>,
    pub cwd: String,
    pub focused: bool,
    // TODO: Hold reference to ManagedPty
    // TODO: Hold block history
}

/// Layout tree for pane arrangement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaneLayout {
    /// A single pane (leaf node).
    Leaf { pane_id: u32 },

    /// Horizontal split (left | right).
    Horizontal {
        left: Box<PaneLayout>,
        right: Box<PaneLayout>,
        /// Split ratio (0.0 - 1.0, proportion of left).
        ratio: f32,
    },

    /// Vertical split (top / bottom).
    Vertical {
        top: Box<PaneLayout>,
        bottom: Box<PaneLayout>,
        /// Split ratio (0.0 - 1.0, proportion of top).
        ratio: f32,
    },
}
