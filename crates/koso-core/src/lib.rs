//! # koso-core
//!
//! Core types, configuration, and shared primitives for Koso.
//!
//! This crate provides:
//! - Configuration loading and management
//! - Shared types (Block, Command, Session)
//! - Event bus for inter-component communication
//! - Error types

pub mod config;
pub mod event;
pub mod types;

pub use config::{config_path, Config, ConfigOverrides};
pub use event::{Event, EventBus};
pub use types::{Block, BlockKind, Command};
