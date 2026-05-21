//! Configuration loading and management.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// CLI overrides that take precedence over the config file.
#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    pub shell: Option<String>,
    pub working_directory: Option<PathBuf>,
}

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

/// Returns the default config file path: `{config_dir}/koso/config.toml`.
///
/// Uses `dirs::config_dir()` for the XDG config directory
/// (`~/.config` on Linux/macOS).
pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join("koso")
        .join("config.toml")
}

impl Config {
    /// Load config from a file path.
    ///
    /// If `path` is `Some`, uses that path directly. If `None`, uses the
    /// default XDG config path returned by [`config_path()`].
    ///
    /// Returns [`Config::default()`] when the file does not exist.
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let config_path = path.unwrap_or_else(config_path);
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .with_context(|| format!("Failed to parse config file at {}", config_path.display()))?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }

    /// Apply CLI overrides to this config. `Some` values replace config values.
    pub fn merge_overrides(mut self, overrides: ConfigOverrides) -> Self {
        if let Some(shell) = overrides.shell {
            self.shell = shell;
        }
        if let Some(wd) = overrides.working_directory {
            self.working_directory = Some(wd);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.shell.is_empty(), "default shell should be non-empty");
        assert!(
            config.working_directory.is_none(),
            "default working_directory should be None"
        );
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = Config::load(Some(PathBuf::from("/nonexistent/config.toml")));
        assert!(result.is_ok(), "loading a nonexistent file should return Ok (default config)");
        let config = result.unwrap();
        assert!(!config.shell.is_empty());
    }

    #[test]
    fn test_load_valid_config() {
        let mut tmp = NamedTempFile::new().expect("failed to create temp file");
        writeln!(
            tmp,
            "shell = \"/bin/bash\"\n[ai]\nprovider = \"openai\""
        )
        .expect("failed to write temp file");

        let config = Config::load(Some(tmp.path().to_path_buf()))
            .expect("failed to load valid config");
        assert_eq!(config.shell, "/bin/bash");
        assert_eq!(config.ai.provider.as_deref(), Some("openai"));
    }

    #[test]
    fn test_load_invalid_toml() {
        let mut tmp = NamedTempFile::new().expect("failed to create temp file");
        writeln!(tmp, "this is not valid toml [[[")
            .expect("failed to write temp file");

        let result = Config::load(Some(tmp.path().to_path_buf()));
        assert!(result.is_err(), "loading invalid TOML should return an error");
        let err_msg = format!("{:#}", result.unwrap_err());
        assert!(
            err_msg.contains("Failed to parse"),
            "error message should contain 'Failed to parse', got: {err_msg}"
        );
    }

    #[test]
    fn test_merge_overrides_shell() {
        let config = Config::default();
        let overrides = ConfigOverrides {
            shell: Some("/bin/fish".into()),
            working_directory: None,
        };
        let merged = config.merge_overrides(overrides);
        assert_eq!(merged.shell, "/bin/fish");
    }

    #[test]
    fn test_merge_overrides_working_directory() {
        let config = Config::default();
        let overrides = ConfigOverrides {
            shell: None,
            working_directory: Some(PathBuf::from("/tmp/work")),
        };
        let merged = config.merge_overrides(overrides);
        assert_eq!(merged.working_directory, Some(PathBuf::from("/tmp/work")));
    }

    #[test]
    fn test_merge_overrides_none_preserves_config() {
        let mut tmp = NamedTempFile::new().expect("failed to create temp file");
        writeln!(tmp, "shell = \"/bin/bash\"").expect("failed to write temp file");

        let config = Config::load(Some(tmp.path().to_path_buf()))
            .expect("failed to load config");
        assert_eq!(config.shell, "/bin/bash");

        let overrides = ConfigOverrides {
            shell: None,
            working_directory: None,
        };
        let merged = config.merge_overrides(overrides);
        assert_eq!(merged.shell, "/bin/bash");
    }

    #[test]
    fn test_config_path_not_empty() {
        let path = config_path();
        let path_str = path.to_string_lossy();
        assert!(
            path_str.ends_with("koso/config.toml"),
            "config_path() should end with 'koso/config.toml', got: {path_str}"
        );
    }
}
