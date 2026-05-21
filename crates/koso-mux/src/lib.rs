//! # koso-mux
//!
//! Terminal multiplexer engine for Koso.
//!
//! Manages pseudo-terminal (PTY) instances, pane layout (splits),
//! and session persistence/restoration.

pub mod pane;
pub mod pty;
pub mod session;

pub use pane::{Pane, PaneLayout};
pub use session::Session;
