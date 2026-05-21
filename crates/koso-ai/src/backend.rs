//! AI backend trait and implementations.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Role in a conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiRole {
    System,
    User,
    Assistant,
}

/// A message in an AI conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: AiRole,
    pub content: String,
}

/// Trait that all AI backends must implement.
pub trait AiBackend: Send + Sync {
    /// Send a prompt and get a response.
    fn complete(&self, messages: &[AiMessage]) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Generate a shell command from natural language.
    fn generate_command(&self, prompt: &str, context: &CommandContext) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Explain terminal output.
    fn explain_output(&self, output: &str) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Assess risk of a command before execution.
    fn assess_risk(&self, command: &str) -> impl std::future::Future<Output = Result<RiskAssessment>> + Send;
}

/// Context provided to the AI for command generation.
#[derive(Debug, Clone, Serialize)]
pub struct CommandContext {
    pub shell: String,
    pub cwd: String,
    pub os: String,
    pub recent_commands: Vec<String>,
}

/// Risk assessment result for a command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Risk level: low, medium, high, critical.
    pub level: RiskLevel,
    /// Human-readable explanation.
    pub explanation: String,
    /// What the command will affect.
    pub affected: Vec<String>,
    /// Whether to require confirmation.
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}
