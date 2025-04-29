// crates/core/src/discover.rs

//! Core discovery module for bitcoin-rpc-codegen.
//!
//! Provides functionality to list RPC methods supported by a given bitcoin-cli binary
//! and to dump their help texts into versioned resource folders.

use serde_json::Value;
use std::{fs, path::Path, process::Command};

use crate::error::{CoreError, DiscoverError};

/// Pure function that parses output from `bitcoin-cli help` into RPC method names.
pub fn parse_help_output(output: &str) -> Vec<String> {
    output
        .lines()
        .filter_map(|line| {
            let name = line.split_whitespace().next()?;
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect()
}

/// Discover available RPC methods by invoking bitcoin-cli help,
/// and dump their help outputs into per-method .txt files under
/// crates/core/resources/docs/vXX/, where vXX is the detected major version.
///
/// On any error (binary missing, non-zero exit, parse failure), it returns
/// an empty Vec as a best-effort fallback.
pub fn discover_methods(bitcoind_bin: &Path) -> Result<Vec<String>, CoreError> {
    // 1) Determine bitcoin-cli path
    let cli_name = if cfg!(windows) {
        "bitcoin-cli.exe"
    } else {
        "bitcoin-cli"
    };
    let cli = bitcoind_bin
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(cli_name);

    // 2) If no cli binary, return empty
    if !cli.exists() {
        return Ok(Vec::new());
    }

    // 3) Run bitcoin-cli getnetworkinfo to detect version
    let output = Command::new(&cli)
        .arg("getnetworkinfo")
        .output()
        .map_err(|e| CoreError::Discover(DiscoverError::CliFailed(e.to_string())))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = extract_major_version(&stdout)?;

    // 4) Prepare the output directory
    let docs_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("docs")
        .join(format!("v{}", version));

    fs::create_dir_all(&docs_dir).map_err(CoreError::Io)?;

    // 5) Run bitcoin-cli help to list available methods
    let output = Command::new(&cli)
        .arg("help")
        .output()
        .map_err(|e| CoreError::Discover(DiscoverError::CliFailed(e.to_string())))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let methods = parse_help_output(&String::from_utf8_lossy(&output.stdout));

    // 6) Dump help text for each method
    for method in &methods {
        match Command::new(&cli).args(["help", method]).output() {
            Ok(output) if output.status.success() => {
                let help_text = String::from_utf8_lossy(&output.stdout);
                let out_path = docs_dir.join(format!("{method}.txt"));
                if let Err(e) = fs::write(&out_path, help_text.as_ref()) {
                    eprintln!("⚠️ Failed to write help for {}: {}", method, e);
                }
            }
            Ok(output) => {
                eprintln!(
                    "⚠️ bitcoin-cli help {} returned non-success status: {}",
                    method, output.status
                );
            }
            Err(e) => {
                eprintln!("⚠️ Failed to run bitcoin-cli help {}: {}", method, e);
            }
        }
    }

    Ok(methods)
}

/// Parse the major version (vXX) from a getnetworkinfo JSON output.
fn extract_major_version(networkinfo_json: &str) -> Result<u32, CoreError> {
    let parsed: Value = serde_json::from_str(networkinfo_json).map_err(|e| {
        CoreError::Discover(DiscoverError::CliFailed(format!(
            "Invalid JSON in getnetworkinfo: {}",
            e
        )))
    })?;

    let version = parsed
        .get("version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| {
            CoreError::Discover(DiscoverError::CliFailed(
                "Missing 'version' field in getnetworkinfo".into(),
            ))
        })?;

    Ok((version / 10000) as u32)
}
