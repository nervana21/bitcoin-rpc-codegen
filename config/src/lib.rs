// config/src/lib.rs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors that can occur when loading or saving configuration
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("Config file not found at: {0}")]
    NotFound(PathBuf),
}

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Bitcoin Core RPC connection settings
    pub bitcoin: BitcoinConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Code generation settings
    pub codegen: CodegenConfig,
}

/// Bitcoin Core RPC connection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    /// RPC host address
    pub host: String,
    /// RPC port
    pub port: u16,
    /// RPC username
    pub username: String,
    /// RPC password
    pub password: String,
    /// Bitcoin Core version (e.g. "v29"); `None` to auto-detect
    pub core_version: Option<String>,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (debug, info, warn, error)
    pub level: String,
    /// Log file path (optional)
    pub file: Option<PathBuf>,
}

/// Code generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodegenConfig {
    /// Path to the `bitcoin-cli help` dump
    pub input_path: PathBuf,
    /// Where to write generated modules
    pub output_dir: PathBuf,
}

impl Config {
    /// Load configuration from a TOML file at `path`
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path.as_ref())?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Save this configuration as a pretty-printed TOML file at `path`
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), ConfigError> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Returns the default config file path:
    /// `{config_dir()}/bitcoin-rpc-codegen/config.toml`
    pub fn default_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("bitcoin-rpc-codegen");
        Ok(config_dir.join("config.toml"))
    }

    /// Get the path to the default help.txt file
    pub fn default_help_path() -> PathBuf {
        PathBuf::from("../resources/help.txt")
    }

    /// Get the default output directory for generated code
    pub fn default_output_dir() -> PathBuf {
        PathBuf::from("../client/src/generated")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bitcoin: BitcoinConfig {
                host: "127.0.0.1".to_string(),
                port: 8332,
                username: "rpcuser".to_string(),
                password: "rpcpassword".to_string(),
                core_version: None,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
            codegen: CodegenConfig {
                input_path: Self::default_help_path(),
                output_dir: Self::default_output_dir(),
            },
        }
    }
}
