//! Configuration loading and management.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Top-level Koso configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Shell to use (e.g., "/bin/zsh", "/bin/bash").
    #[serde(default = "default_shell")]
    pub shell: String,

    /// Default working directory.
    pub working_directory: Option<PathBuf>,

    /// AI backend configuration.
    #[serde(default)]
    pub ai: AiConfig,

    /// Keybinding overrides.
    #[serde(default)]
    pub keys: KeyConfig,

    /// Theme configuration.
    #[serde(default)]
    pub theme: ThemeConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiConfig {
    /// AI provider: "openai", "anthropic", "local", "opencode".
    pub provider: Option<String>,

    /// API key (prefer env var KOSO_AI_KEY).
    pub api_key: Option<String>,

    /// Model name.
    pub model: Option<String>,

    /// Base URL for custom endpoints.
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyConfig {
    // TODO: Define keybinding overrides
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeConfig {
    // TODO: Define theme options
}

fn default_shell() -> String {
    std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
}

impl Default for Config {
    fn default() -> Self {
        Self {
            shell: default_shell(),
            working_directory: None,
            ai: AiConfig::default(),
            keys: KeyConfig::default(),
            theme: ThemeConfig::default(),
        }
    }
}

impl Config {
    /// Load config from the default location (~/.config/koso/config.toml).
    pub fn load() -> Result<Self> {
        let config_path = dirs_config_path();
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}

fn dirs_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("koso")
        .join("config.toml")
}
