//! # koso-ai
//!
//! AI integration layer for Koso.
//!
//! Provides a trait-based abstraction over AI backends, allowing
//! pluggable providers (OpenAI, Anthropic, local models, OpenCode).
//!
//! Key capabilities:
//! - Natural language → command generation
//! - Output explanation ("what does this error mean?")
//! - Output transformation ("extract the IPs from this output")
//! - Risk assessment for destructive commands

pub mod backend;
pub mod prompt;

pub use backend::{AiBackend, AiMessage, AiRole};
