// examples/download_v29.rs

use std::{
    env,
    fs::{create_dir_all, File},
    io::{copy, BufReader},
    path::PathBuf,
    process::Command,
};

use flate2::read::GzDecoder;
use tar::Archive;

/// Hardcoded: Download URL and filename for macOS x86_64 (v29.0)
const URL: &str =
    "https://bitcoincore.org/bin/bitcoin-core-29.0/bitcoin-29.0-x86_64-apple-darwin.tar.gz";
const FILENAME: &str = "bitcoin-29.0-x86_64-apple-darwin.tar.gz";
const VERSION_DIR: &str = "bitcoin-versions/v29";
const EXTRACT_DIR: &str = "bitcoin-29.0";

fn main() -> anyhow::Result<()> {
    let home = env::var("HOME")?;
    let target_dir = PathBuf::from(&home).join(VERSION_DIR);
    let archive_path = target_dir.join(FILENAME);

    // Step 1: Create directory if it doesn't exist
    create_dir_all(&target_dir)?;

    // Step 2: Download the .tar.gz if it doesn’t exist yet
    if !archive_path.exists() {
        println!("⬇️ Downloading {}", FILENAME);
        let mut resp = reqwest::blocking::get(URL)?;
        let mut out = File::create(&archive_path)?;
        copy(&mut resp, &mut out)?;
        println!("✅ Downloaded to {}", archive_path.display());
    } else {
        println!("⚠️  File already exists: {}", archive_path.display());
    }

    // Step 3: Extract the archive
    let extract_path = target_dir.join(EXTRACT_DIR);
    if extract_path.exists() {
        println!("📦 Already extracted: {}", extract_path.display());
    } else {
        println!("📂 Extracting to {}", extract_path.display());
        let tar_gz = File::open(&archive_path)?;
        let decompressed = GzDecoder::new(BufReader::new(tar_gz));
        let mut archive = Archive::new(decompressed);
        archive.unpack(&target_dir)?;
        println!("✅ Extraction complete");
    }

    // Step 4: Show resulting path to bitcoind
    let bin_path = extract_path.join("bin/bitcoind");
    println!("🚀 bitcoind path: {}", bin_path.display());

    // Optional: Make sure it works
    let version_output = Command::new(&bin_path).arg("--version").output()?;

    if version_output.status.success() {
        println!(
            "🔍 bitcoind version:\n{}",
            String::from_utf8_lossy(&version_output.stdout)
        );
    } else {
        println!(
            "❌ Error running bitcoind:\n{}",
            String::from_utf8_lossy(&version_output.stderr)
        );
    }

    Ok(())
}
