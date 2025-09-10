// config/src/lib.rs

use anyhow::Result;
use bitcoin::Network;
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
    /// Bitcoin network to use
    pub network: Option<Network>,
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
    /// Path to the API schema file (api.json)
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

    /// Get the default output directory for generated code
    pub fn default_output_dir() -> PathBuf {
        Self::default_output_dir_internal(
            std::env::var("OUT_DIR").ok(),
            std::env::current_dir().ok(),
        )
    }

    /// Internal function for testing - allows injection of environment values
    fn default_output_dir_internal(
        out_dir: Option<String>,
        current_dir: Option<PathBuf>,
    ) -> PathBuf {
        // First try to get OUT_DIR environment variable
        if let Some(out_dir) = out_dir {
            return PathBuf::from(out_dir);
        }

        // Fallback to current directory
        if let Some(current_dir) = current_dir {
            return current_dir;
        }

        // Last resort - use current directory as string
        PathBuf::from(".")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bitcoin: BitcoinConfig {
                host: "127.0.0.1".to_string(),
                port: 18443,
                username: "rpcuser".to_string(),
                password: "rpcpassword".to_string(),
                network: None,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
            codegen: CodegenConfig {
                input_path: PathBuf::from("api.json"),
                output_dir: Self::default_output_dir(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_from_file() {
        // Test successful loading with explicit TOML content
        let temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
            [bitcoin]
            host = "127.0.0.1"
            port = 18443
            username = "rpcuser"
            password = "rpcpassword"
            
            [logging]
            level = "info"
            
            [codegen]
            input_path = "api.json"
            output_dir = "generated"
        "#;
        fs::write(&temp_file, toml_content).unwrap();

        let loaded_config = Config::from_file(&temp_file).unwrap();
        assert_eq!(loaded_config.bitcoin.host, "127.0.0.1");
        assert_eq!(loaded_config.bitcoin.port, 18443);
        assert_eq!(loaded_config.bitcoin.username, "rpcuser");
        assert_eq!(loaded_config.bitcoin.password, "rpcpassword");
        assert_eq!(loaded_config.logging.level, "info");
        assert_eq!(loaded_config.codegen.input_path, PathBuf::from("api.json"));
        assert_eq!(loaded_config.codegen.output_dir, PathBuf::from("generated"));

        // Test successful parsing with different content
        let temp_file2 = NamedTempFile::new().unwrap();
        let toml_content2 = r#"
            [bitcoin]
            host = "localhost"
            port = 8332
            username = "testuser"
            password = "testpass"
            
            [logging]
            level = "debug"
            file = "debug.log"
            
            [codegen]
            input_path = "test_api.json"
            output_dir = "test_generated"
        "#;
        fs::write(&temp_file2, toml_content2).unwrap();

        let loaded_config2 = Config::from_file(&temp_file2).unwrap();
        assert_eq!(loaded_config2.bitcoin.host, "localhost");
        assert_eq!(loaded_config2.bitcoin.port, 8332);
        assert_eq!(loaded_config2.bitcoin.username, "testuser");
        assert_eq!(loaded_config2.bitcoin.password, "testpass");
        assert_eq!(loaded_config2.logging.level, "debug");
        assert_eq!(
            loaded_config2.logging.file,
            Some(PathBuf::from("debug.log"))
        );
        assert_eq!(
            loaded_config2.codegen.input_path,
            PathBuf::from("test_api.json")
        );
        assert_eq!(
            loaded_config2.codegen.output_dir,
            PathBuf::from("test_generated")
        );

        // Test file not found error
        let result = Config::from_file("nonexistent_file.toml");
        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::FileRead(_) => {}
            _ => panic!("Expected FileRead error"),
        }

        // Test parse error
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(&temp_file, "invalid toml content").unwrap();

        let result = Config::from_file(&temp_file);
        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::Parse(_) => {}
            _ => panic!("Expected Parse error"),
        }
    }

    #[test]
    fn test_save() {
        let config = Config::default();
        let temp_file = NamedTempFile::new().unwrap();

        // Test successful save
        let result = config.save(&temp_file);
        assert!(result.is_ok());

        // Verify the file was written and can be read back
        let contents = fs::read_to_string(&temp_file).unwrap();
        assert!(contents.contains("127.0.0.1"));
        assert!(contents.contains("18443"));
        assert!(contents.contains("rpcuser"));
        assert!(contents.contains("rpcpassword"));
        assert!(contents.contains("info"));

        // Test file write error - try to save to a non-existent directory
        let temp_dir = tempfile::tempdir().unwrap();
        let non_existent_subdir = temp_dir.path().join("nonexistent").join("config.toml");

        let result = config.save(&non_existent_subdir);
        assert!(result.is_err());

        // Verify the error is a FileRead error (from std::fs::write)
        match result.unwrap_err() {
            ConfigError::FileRead(_) => (), // Expected
            other => panic!("Expected FileRead error, got {:?}", other),
        }
    }

    #[test]
    fn test_default_path() {
        let path = Config::default_path().unwrap();
        assert!(path
            .to_str()
            .unwrap()
            .ends_with("bitcoin-rpc-codegen/config.toml"));

        // Test that the path contains the expected directory structure
        let path_str = path.to_str().unwrap();
        assert!(path_str.contains("bitcoin-rpc-codegen"));
        assert!(path_str.ends_with("config.toml"));
    }

    #[test]
    fn test_default_output_dir() {
        let dir = Config::default_output_dir();

        // Test that we get a valid path
        assert!(dir.to_str().is_some());

        // Test OUT_DIR environment variable path
        std::env::set_var("OUT_DIR", "/tmp/test_out_dir");
        let dir_with_env = Config::default_output_dir();
        assert_eq!(dir_with_env.to_str().unwrap(), "/tmp/test_out_dir");

        // Clean up
        std::env::remove_var("OUT_DIR");

        // Test fallback to current directory
        std::env::remove_var("OUT_DIR");
        let temp_dir = tempfile::tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        let dir = Config::default_output_dir();
        let canonical_temp_path = temp_dir.path().canonicalize().unwrap();
        assert_eq!(dir, canonical_temp_path);
        std::env::set_current_dir(original_dir).unwrap();

        // Test last resort fallback
        std::env::remove_var("OUT_DIR");
        let dir = Config::default_output_dir();
        assert!(dir.to_str().is_some());
        assert!(!dir.to_str().unwrap().is_empty());
    }

    #[test]
    fn test_default_output_dir_internal() {
        // Test OUT_DIR takes precedence
        let dir = Config::default_output_dir_internal(
            Some("/tmp/out_dir".to_string()),
            Some(PathBuf::from("/tmp/current")),
        );
        assert_eq!(dir, PathBuf::from("/tmp/out_dir"));

        // Test current_dir fallback when OUT_DIR is None
        let dir = Config::default_output_dir_internal(None, Some(PathBuf::from("/tmp/current")));
        assert_eq!(dir, PathBuf::from("/tmp/current"));

        // Test the fallback case by passing None for both parameters
        let dir = Config::default_output_dir_internal(None, None);
        assert_eq!(dir, PathBuf::from("."));
    }

    #[test]
    fn test_default() {
        let config = Config::default();
        assert_eq!(config.bitcoin.host, "127.0.0.1");
        assert_eq!(config.bitcoin.port, 18443);
        assert_eq!(config.bitcoin.username, "rpcuser");
        assert_eq!(config.bitcoin.password, "rpcpassword");
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.logging.file, None);
        assert_eq!(config.codegen.input_path, PathBuf::from("api.json"));
        // output_dir is dynamic, so we just check it's not empty
        assert!(!config.codegen.output_dir.to_str().unwrap().is_empty());
    }
}
