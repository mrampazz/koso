//! PTY (pseudo-terminal) management.
//!
//! Wraps `portable-pty` to spawn shell processes and manage I/O.

use anyhow::Result;

/// A managed PTY instance.
pub struct ManagedPty {
    // TODO: Hold the PTY master/child pair
    // TODO: Manage reader/writer channels
}

impl ManagedPty {
    /// Spawn a new PTY with the given shell command.
    pub fn spawn(_shell: &str, _cwd: &str, _cols: u16, _rows: u16) -> Result<Self> {
        // TODO: Use portable-pty to create a PTY pair
        // TODO: Spawn the shell process
        // TODO: Set up async readers for stdout
        todo!("PTY spawn not yet implemented")
    }

    /// Resize the PTY.
    pub fn resize(&self, _cols: u16, _rows: u16) -> Result<()> {
        todo!("PTY resize not yet implemented")
    }

    /// Write input to the PTY.
    pub fn write(&self, _data: &[u8]) -> Result<()> {
        todo!("PTY write not yet implemented")
    }
}
