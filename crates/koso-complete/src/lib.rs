//! # koso-complete
//!
//! Context-aware autocompletion engine for Koso.
//!
//! Provides completions by parsing live system state:
//! - File paths (with fuzzy matching)
//! - Git branches, tags, remotes
//! - Docker containers, images
//! - Kubernetes resources (pods, services, etc.)
//! - Command flags and subcommands (from man pages / --help)
//! - Environment variables
//! - Command history (frecency-ranked)

pub mod engine;
pub mod provider;

pub use engine::CompletionEngine;
pub use provider::{Completion, CompletionProvider};
