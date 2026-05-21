//! Completion engine — aggregates results from multiple providers.

use anyhow::Result;
use crate::provider::{Completion, CompletionProvider};

/// The main completion engine that orchestrates multiple providers.
pub struct CompletionEngine {
    providers: Vec<Box<dyn CompletionProvider>>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
        }
    }

    /// Register a completion provider.
    pub fn register(&mut self, provider: Box<dyn CompletionProvider>) {
        self.providers.push(provider);
    }

    /// Get completions for the current input, aggregated from all providers.
    pub fn complete(&self, input: &str, cursor_pos: usize, cwd: &str) -> Result<Vec<Completion>> {
        let mut all_completions = Vec::new();

        for provider in &self.providers {
            if provider.can_complete(input, cursor_pos) {
                match provider.complete(input, cursor_pos, cwd) {
                    Ok(completions) => all_completions.extend(completions),
                    Err(e) => {
                        tracing::warn!(
                            "Completion provider '{}' failed: {}",
                            provider.name(),
                            e
                        );
                    }
                }
            }
        }

        // Sort by score descending.
        all_completions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        Ok(all_completions)
    }
}

impl Default for CompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}
