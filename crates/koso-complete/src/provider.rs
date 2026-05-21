//! Completion provider trait and types.

use anyhow::Result;

/// A single completion suggestion.
#[derive(Debug, Clone)]
pub struct Completion {
    /// The text to insert.
    pub text: String,

    /// Display label (may differ from inserted text).
    pub label: String,

    /// Optional description/documentation.
    pub description: Option<String>,

    /// Source of the completion (e.g., "git", "path", "history").
    pub source: String,

    /// Relevance score (higher = more relevant).
    pub score: f64,
}

/// Trait for completion providers.
///
/// Each provider is responsible for one domain of completions
/// (e.g., file paths, git branches, command flags).
pub trait CompletionProvider: Send + Sync {
    /// Name of this provider.
    fn name(&self) -> &str;

    /// Whether this provider can handle the given input context.
    fn can_complete(&self, input: &str, cursor_pos: usize) -> bool;

    /// Generate completions for the given input.
    fn complete(&self, input: &str, cursor_pos: usize, cwd: &str) -> Result<Vec<Completion>>;
}
