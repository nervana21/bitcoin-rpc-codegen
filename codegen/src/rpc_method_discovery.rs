//! RPC Method Discovery module for bitcoin-rpc-codegen.
//!
//! Provides functionality to list RPC methods supported by a given bitcoin-cli binary
//! and to dump their help texts into versioned resource folders.

use rpc_api::{ApiArgument, ApiMethod, ApiResult};
use serde_json::Value;
use std::{fs, path::Path, process::Command};

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
/// resources/methods/vXX/, where vXX is the detected major version.
/// Also maintains a 'latest' symlink to the most recent version.
///
/// On any error (binary missing, non-zero exit, parse failure), it returns
/// an empty Vec as a best-effort fallback.
pub fn discover_methods(bitcoind_bin: &Path) -> Result<Vec<ApiMethod>, String> {
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
        .map_err(|e| format!("Failed to run bitcoin-cli: {}", e))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = extract_major_version(&stdout)?;

    // 4) Prepare the output directories
    let methods_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("methods");

    let version_dir = methods_dir.join(format!("v{}", version));
    let latest_dir = methods_dir.join("latest");

    // Create the methods directory if it doesn't exist
    fs::create_dir_all(&methods_dir)
        .map_err(|e| format!("Failed to create methods directory: {}", e))?;

    // Create the version-specific directory
    fs::create_dir_all(&version_dir)
        .map_err(|e| format!("Failed to create version directory: {}", e))?;

    // Remove existing latest symlink if it exists
    if latest_dir.exists() {
        fs::remove_file(&latest_dir)
            .map_err(|e| format!("Failed to remove existing latest symlink: {}", e))?;
    }

    // Create new latest symlink
    #[cfg(unix)]
    std::os::unix::fs::symlink(&version_dir, &latest_dir)
        .map_err(|e| format!("Failed to create latest symlink: {}", e))?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&version_dir, &latest_dir)
        .map_err(|e| format!("Failed to create latest symlink: {}", e))?;

    // 5) Run bitcoin-cli help to list available methods
    let output = Command::new(&cli)
        .arg("help")
        .output()
        .map_err(|e| format!("Failed to run bitcoin-cli help: {}", e))?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let method_names = parse_help_output(&String::from_utf8_lossy(&output.stdout));
    let mut methods = Vec::new();

    // 6) Dump help text for each method and create ApiMethod
    for method_name in method_names {
        match Command::new(&cli).args(["help", &method_name]).output() {
            Ok(output) if output.status.success() => {
                let help_text = String::from_utf8_lossy(&output.stdout);
                let out_path = version_dir.join(format!("{}.txt", method_name));
                if let Err(e) = fs::write(&out_path, help_text.as_ref()) {
                    eprintln!("⚠️ Failed to write help for {}: {}", method_name, e);
                }

                // Create ApiMethod from help text
                methods.push(ApiMethod {
                    name: method_name,
                    description: help_text.to_string(),
                    arguments: parse_help_text(&help_text).1,
                    results: parse_help_text(&help_text).2,
                });
            }
            Ok(output) => {
                eprintln!(
                    "⚠️ bitcoin-cli help {} returned non-success status: {}",
                    method_name, output.status
                );
            }
            Err(e) => {
                eprintln!("⚠️ Failed to run bitcoin-cli help {}: {}", method_name, e);
            }
        }
    }

    Ok(methods)
}

/// Parse the major version (vXX) from a getnetworkinfo JSON output.
fn extract_major_version(networkinfo_json: &str) -> Result<u32, String> {
    let parsed: Value = serde_json::from_str(networkinfo_json)
        .map_err(|e| format!("Invalid JSON in getnetworkinfo: {}", e))?;

    let version = parsed
        .get("version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "Missing 'version' field in getnetworkinfo".to_string())?;

    Ok((version / 10000) as u32)
}

fn parse_help_text(help_text: &str) -> (String, Vec<ApiArgument>, Vec<ApiResult>) {
    let mut sections = help_text.split("\n\n");

    // Parse description (everything before Arguments:)
    let description = sections
        .next()
        .unwrap_or("")
        .lines()
        .collect::<Vec<_>>()
        .join("\n");

    // Parse arguments
    let mut arguments = Vec::new();
    if let Some(args_section) = sections.find(|s| s.starts_with("Arguments:")) {
        for line in args_section.lines().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            if let Some((_num, rest)) = line.split_once('.') {
                if let Some((name_type, desc)) = rest.split_once("(") {
                    let name = name_type
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string();
                    let type_ = name_type
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or("string")
                        .trim_matches(|c| c == '(' || c == ')')
                        .to_string();
                    let optional = desc.contains("optional");
                    arguments.push(ApiArgument {
                        names: vec![name],
                        type_,
                        optional,
                        description: desc.trim_matches(|c| c == '(' || c == ')').to_string(),
                    });
                }
            }
        }
    }

    // Parse results
    let mut results = Vec::new();
    if let Some(results_section) = sections.find(|s| s.starts_with("Result:")) {
        for line in results_section.lines().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            if let Some((name_type, desc)) = line.split_once("(") {
                let name = name_type
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_string();
                let type_ = name_type
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("string")
                    .trim_matches(|c| c == '(' || c == ')')
                    .to_string();
                let optional = desc.contains("optional");
                results.push(ApiResult {
                    key_name: name,
                    type_,
                    description: desc.trim_matches(|c| c == '(' || c == ')').to_string(),
                    inner: Vec::new(),
                    optional,
                });
            }
        }
    }

    (description, arguments, results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_discover_methods_file_generation() {
        // Create a temporary directory for our test
        let temp_dir = TempDir::new().unwrap();

        // Create a mock bitcoin-cli binary that returns known responses
        let mock_cli_path = create_mock_bitcoin_cli(&temp_dir);

        // Run discover_methods
        let result = discover_methods(&mock_cli_path);
        assert!(result.is_ok());

        // Check that the files were created in the correct location
        let methods_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("methods");

        // Check version directory exists
        let version_dir = methods_dir.join("v25"); // Assuming v25 from mock response
        assert!(version_dir.exists());

        // Check latest symlink exists and points to version directory
        let latest_dir = methods_dir.join("latest");
        assert!(latest_dir.exists());

        // Check that some expected method files exist
        let expected_methods = ["getblockcount", "getblockhash"];
        for method in expected_methods {
            let method_file = version_dir.join(format!("{}.txt", method));
            assert!(method_file.exists(), "Method file {} should exist", method);
        }

        // Clean up
        fs::remove_dir_all(&methods_dir).unwrap();
    }

    fn create_mock_bitcoin_cli(temp_dir: &TempDir) -> PathBuf {
        let mock_cli_path = temp_dir.path().join(if cfg!(windows) {
            "bitcoin-cli.exe"
        } else {
            "bitcoin-cli"
        });

        // Create a shell script that returns mock responses
        let script_content = if cfg!(windows) {
            r#"@echo off
if "%1"=="getnetworkinfo" (
    echo {"version": 250000}
) else if "%1"=="help" (
    echo getblockcount
    echo getblockhash
) else if "%1"=="help" "%2" (
    echo Help text for %2
)
"#
        } else {
            r#"#!/bin/sh
if [ "$1" = "getnetworkinfo" ]; then
    echo '{"version": 250000}'
elif [ "$1" = "help" ] && [ -z "$2" ]; then
    echo "getblockcount"
    echo "getblockhash"
elif [ "$1" = "help" ] && [ -n "$2" ]; then
    echo "Help text for $2"
fi
"#
        };

        fs::write(&mock_cli_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&mock_cli_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        mock_cli_path
    }
}
