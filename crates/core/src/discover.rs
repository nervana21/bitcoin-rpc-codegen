// crates/core/src/discover.rs

//! Core discovery module for bitcoin-rpc-codegen.
//!
//! Provides functionality to list RPC methods supported by a given bitcoin-cli binary.

use anyhow::{Context, Result};
use std::{path::Path, process::Command};

/// Discover available RPC methods by invoking `bitcoin-cli help`.
///
/// Given a path to a bitcoind binary, this function will look for a sibling
/// `bitcoin-cli` (in the same directory) and run:
///
/// ```text
/// bitcoin-cli help
/// ```
///
/// On any error (binary missing, non-zero exit, parse failure), it returns
/// an empty Vec as a best-effort fallback.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use bitcoin_rpc_codegen::discover_methods;
///
/// // If your bitcoind is at /usr/local/bin/bitcoind,
/// // then bitcoin-cli is expected next to it.
/// let methods = discover_methods(Path::new("/usr/local/bin/bitcoind")).unwrap();
/// assert!(methods.iter().all(|m| !m.is_empty()));
/// ```
pub fn discover_methods(bitcoind_bin: &Path) -> Result<Vec<String>> {
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

    // 3) Run `bitcoin-cli help`
    let output = Command::new(cli)
        .arg("help")
        .output()
        .context("failed to execute `bitcoin-cli help`")?;

    // 4) Non-zero exit status => no methods
    if !output.status.success() {
        return Ok(Vec::new());
    }

    // 5) Parse each line's first token if it's a valid RPC method name
    let stdout = String::from_utf8_lossy(&output.stdout);
    let methods = stdout
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
