// crates/core/src/fetch.rs

//! Download module for bitcoin-rpc-codegen.

use anyhow::Result;
use flate2::read::GzDecoder;
use reqwest::blocking::Client;
use std::{
    env,
    fs::{create_dir_all, File},
    io::copy,
    path::PathBuf,
};
use tar::Archive;

use crate::error::FetchError;

/// Download or locate the `bitcoind` binary for `version`.
///
/// Checks `BITCOIND_PATH` env var first; otherwise:
/// 1. Normalizes `version` ("v29" → "29.0", "29" → "29.0", "0.21.1" → "0.21.1")  
/// 2. Builds URL:
///    `https://bitcoincore.org/bin/bitcoin-core-<ver>/bitcoin-<ver>-<platform>.tar.gz`  
/// 3. Downloads into `~/bitcoin-versions/v<major>/…`  
/// 4. Unpacks and returns `…/bitcoin-<ver>/bin/bitcoind`
///
/// # Examples
///
/// ```no_run
/// use std::env;
/// use std::path::PathBuf;
/// use bitcoin_rpc_codegen::fetch::fetch_bitcoind;
///
/// // 1) Env override takes precedence
/// env::set_var("BITCOIND_PATH", "/usr/local/bin/bitcoind");
/// assert_eq!(
///     fetch_bitcoind("v123").unwrap(),
///     PathBuf::from("/usr/local/bin/bitcoind")
/// );
///
/// // 2) Missing BITCOIND_PATH + unsupported version errors
/// env::remove_var("BITCOIND_PATH");
/// assert!( fetch_bitcoind("v0").is_err() );
/// ```
///
pub fn fetch_bitcoind(version: &str) -> Result<PathBuf, FetchError> {
    // 1) Check override
    if let Ok(path) = env::var("BITCOIND_PATH") {
        return Ok(PathBuf::from(path));
    }

    // 2) Normalize version
    let clean = version.strip_prefix('v').unwrap_or(version);
    let semver = if clean.contains('.') {
        clean.to_string()
    } else {
        format!("{}.0", clean)
    };
    let major = semver.split('.').next().unwrap_or(&semver);

    // 3) Determine platform
    let platform = default_platform();

    // 4) Construct download URL
    let filename = format!("bitcoin-{}-{}.tar.gz", semver, platform);
    let url = format!(
        "https://bitcoincore.org/bin/bitcoin-core-{ver}/bitcoin-{ver}-{plat}.tar.gz",
        ver = semver,
        plat = platform
    );

    // 5) Prepare local paths
    let home = env::var("HOME").map_err(|e| {
        FetchError::DownloadFailed(format!("HOME environment variable not set: {}", e))
    })?;
    let base_dir = PathBuf::from(home)
        .join("bitcoin-versions")
        .join(format!("v{}", major));
    create_dir_all(&base_dir).map_err(|e| {
        FetchError::DownloadFailed(format!("Failed to create directory {:?}: {}", &base_dir, e))
    })?;

    let archive_path = base_dir.join(&filename);
    let extract_dir = base_dir.join(format!("bitcoin-{}", semver));

    // 6) Download if missing
    if !archive_path.exists() {
        println!("⬇️ Downloading {}...", url);
        let mut resp = Client::new()
            .get(&url)
            .send()
            .map_err(|e| FetchError::DownloadFailed(format!("Failed HTTP GET {}: {}", url, e)))?
            // Unify context: treat status errors same as connection errors
            .error_for_status()
            .map_err(|e| FetchError::DownloadFailed(format!("Failed HTTP GET {}: {}", url, e)))?;
        let mut out_file = File::create(&archive_path).map_err(|e| {
            FetchError::DownloadFailed(format!("Failed to create file {:?}: {}", &archive_path, e))
        })?;
        copy(&mut resp, &mut out_file).map_err(|e| {
            FetchError::DownloadFailed(format!("Failed writing download to disk: {}", e))
        })?;
        println!("✅ Downloaded to {}", archive_path.display());
    }

    // 7) Extract if missing
    if !extract_dir.exists() {
        println!("📂 Extracting {:?} to {:?}...", archive_path, base_dir);
        let tar_gz = File::open(&archive_path).map_err(|e| {
            FetchError::ExtractFailed(format!("Failed to open archive {:?}: {}", &archive_path, e))
        })?;
        let decoder = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(decoder);
        archive.unpack(&base_dir).map_err(|e| {
            FetchError::ExtractFailed(format!(
                "Failed to unpack archive into {:?}: {}",
                &base_dir, e
            ))
        })?;
        println!("✅ Extraction complete");
    }

    // 8) Return binary path
    let bitcoind_path = extract_dir.join("bin").join("bitcoind");
    if !bitcoind_path.exists() {
        return Err(FetchError::NotFound(bitcoind_path));
    }

    Ok(bitcoind_path)
}

/// Detect host platform for Bitcoin Core release downloads.
pub fn default_platform() -> String {
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        "x86_64-apple-darwin".into()
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        "arm64-apple-darwin".into()
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        "x86_64-linux-gnu".into()
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        "aarch64-linux-gnu".into()
    }
    #[cfg(all(windows, target_arch = "x86_64"))]
    {
        "x86_64-w64-mingw32".into()
    }
    // Fallback
    #[cfg(not(any(
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(windows, target_arch = "x86_64"),
    )))]
    {
        "x86_64-unknown-unknown".into()
    }
}
