// examples/discover.rs
//
// Discover RPC methods and dump help output for each.

use anyhow::{bail, Context, Result};
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let mut version: Option<String> = None;
    let mut bin_path: Option<String> = None;

    let mut args = env::args().skip(1); // Skip program name

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--version" => {
                version = Some(
                    args.next()
                        .context("Expected version after --version (e.g., v29)")?,
                );
            }
            "--bin-path" => {
                bin_path = Some(args.next().context("Expected path after --bin-path")?);
            }
            unknown => {
                bail!("Unknown argument: {}", unknown);
            }
        }
    }

    let version = version.context("Missing required --version argument (e.g., --version v29)")?;
    let bin_path = bin_path.context("Missing required --bin-path argument")?;

    if !Path::new(&bin_path).exists() {
        bail!("bitcoind binary not found at: {}", bin_path);
    }

    println!(
        "🚀 Discovering methods for version {} using binary {}",
        version, bin_path
    );

    // Proceed to discovery logic...
    discover_methods(&bin_path, &version)?;

    println!("✅ Discovery complete.");
    Ok(())
}

fn discover_methods(bin_path: &str, version: &str) -> Result<()> {
    // Your discovery logic here (example)
    println!(
        "(Pretend) discovering methods from {} for {}",
        bin_path, version
    );
    Ok(())
}
