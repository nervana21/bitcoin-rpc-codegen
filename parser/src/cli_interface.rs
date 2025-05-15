// parser/src/cli_interface.rs

use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("CLI command failed: {0}")]
    CommandFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid JSON: {0}")]
    JsonError(String),
    #[error("Missing version field in getnetworkinfo")]
    MissingVersion,
}

/// Interface for interacting with bitcoin-cli
pub struct BitcoinCli {
    cli_path: PathBuf,
}

impl BitcoinCli {
    /// Create a new BitcoinCli interface
    pub fn new(bitcoind_bin: &Path) -> Self {
        let cli_name = if cfg!(windows) {
            "bitcoin-cli.exe"
        } else {
            "bitcoin-cli"
        };
        let cli = bitcoind_bin
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(cli_name);

        Self { cli_path: cli }
    }

    /// Get the major version of the Bitcoin node
    pub fn get_version(&self) -> Result<u32, CliError> {
        let output = Command::new(&self.cli_path)
            .arg("getnetworkinfo")
            .output()
            .map_err(|e| CliError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(CliError::CommandFailed("getnetworkinfo failed".into()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let parsed: Value = serde_json::from_str(&stdout)
            .map_err(|e| CliError::JsonError(format!("Invalid JSON in getnetworkinfo: {}", e)))?;

        let version = parsed
            .get("version")
            .and_then(|v| v.as_u64())
            .ok_or(CliError::MissingVersion)?;

        Ok((version / 10000) as u32)
    }

    /// Get list of available RPC methods
    pub fn get_methods(&self) -> Result<Vec<String>, CliError> {
        let output = Command::new(&self.cli_path)
            .arg("help")
            .output()
            .map_err(|e| CliError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(CliError::CommandFailed("help command failed".into()));
        }

        let methods = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter_map(|line| {
                let name = line.split_whitespace().next()?;
                if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(methods)
    }

    /// Get help text for a specific method
    pub fn get_help_text(&self, method: &str) -> Result<String, CliError> {
        let output = Command::new(&self.cli_path)
            .args(["help", method])
            .output()
            .map_err(|e| CliError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            return Err(CliError::CommandFailed(format!("help {} failed", method)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }

    /// Save help text for all methods to a directory
    pub fn save_help_texts(&self, output_dir: &Path) -> Result<(), CliError> {
        let version = self.get_version()?;
        let docs_dir = output_dir.join(format!("v{}", version));
        fs::create_dir_all(&docs_dir)?;

        for method in self.get_methods()? {
            let help_text = self.get_help_text(&method)?;
            let out_path = docs_dir.join(format!("{}.txt", method));
            fs::write(&out_path, help_text)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_help_output() {
        let input = "getblockcount\ngetblockhash <height>\ninvalid@method";
        let methods: Vec<String> = input
            .lines()
            .filter_map(|line| {
                let name = line.split_whitespace().next()?;
                if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(methods, vec!["getblockcount", "getblockhash"]);
    }

    #[test]
    fn test_extract_major_version() {
        let input = r#"{"version": 250000}"#;
        let parsed: Value = serde_json::from_str(input).unwrap();
        let version = parsed.get("version").and_then(|v| v.as_u64()).unwrap();
        assert_eq!((version / 10000) as u32, 25);
    }

    #[test]
    fn test_invalid_binary() {
        let cli = BitcoinCli::new(Path::new("/nonexistent/path"));
        assert!(cli.get_version().is_err());
    }
}
