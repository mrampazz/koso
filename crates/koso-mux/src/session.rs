//! Session persistence and restoration.
//!
//! Allows saving the current pane layout, working directories,
//! and optionally scroll-back history to disk for later restoration.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::PaneLayout;

/// A serializable session snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub layout: PaneLayout,
    pub created_at: String,
    pub working_directories: Vec<String>,
}

impl Session {
    /// Save session to disk.
    pub fn save(&self, _path: &PathBuf) -> Result<()> {
        // TODO: Serialize to JSON and write to file
        todo!("Session save not yet implemented")
    }

    /// Load session from disk.
    pub fn load(_path: &PathBuf) -> Result<Self> {
        // TODO: Read file and deserialize
        todo!("Session load not yet implemented")
    }
}
