// examples/bootstrap_v29.rs
//
// Full v29 bootstrap pipeline:
//  0. (Optional) verify_all_methods_v29 to populate feedback
//  1. discover               ‣ dump help texts (only if missing or forced)
//  2. extract_api_v29         ‣ build raw JSON schema
//  3. regenerate_schema_v29   ‣ canonicalize schema
//  4. validate_roundtrip_v29  ‣ zero-arg roundtrip sanity check
//  5. generate_v29            ‣ emit Rust client + types
//
// Invocation:
//   cargo run --example bootstrap_v29
//   cargo run --example bootstrap_v29 -- --force  (force re-discover and feedback regeneration)

use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    const VERSION: &str = "v29";

    let home = env::var("HOME").context("Failed to get $HOME env var")?;
    let bin_path = format!(
        "{}/bitcoin-versions/{}/bitcoin-{}.0/bin/bitcoind",
        home,
        &VERSION[1..],
        &VERSION[1..]
    );

    println!("🚀 Starting bootstrap for version: {}", VERSION);

    // --- 📂 Parse CLI args ---
    let force = env::args().any(|arg| arg == "--force");
    if force {
        println!("⚡ Force mode enabled — will re-run discovery and feedback steps.");
    }

    // --- 📂 Step 0: Ensure feedback/ is populated ---
    if force || !Path::new("feedback").exists() {
        println!("📂 Running `verify_all_methods_v29` to (re)generate feedback...");
        run_example("verify_all_methods_v29", &[])?;
        println!("✅ `feedback/` generated successfully.");
    } else {
        println!("📂 `feedback/` directory already exists. Skipping feedback generation.");
    }

    // --- 📂 Step 0b: Ensure v29_docs/ exists by checking index.txt ---
    if force || !Path::new("resources/v29_docs/index.txt").exists() {
        println!("📂 Running `discover` to (re)generate method docs...");
        run_example("discover", &["--bin-path", &bin_path])?;
        println!("✅ `resources/v29_docs/` generated successfully.");
    } else {
        println!("📂 `resources/v29_docs/index.txt` already exists. Skipping discovery.");
    }

    // --- 🚀 Main pipeline steps ---
    let steps = &[
        ("extract_api_v29", &[][..]),
        ("regenerate_schema_v29", &[][..]),
        ("validate_roundtrip_v29", &[][..]),
        ("generate_v29", &[][..]),
    ];

    for (name, extra) in steps {
        println!("\n=== STEP `{}` ===", name);
        run_example(name, extra)?;
    }

    println!("\n✅ bootstrap_v29 complete — all steps ran successfully and deterministically!");
    Ok(())
}

/// Helper to run a cargo example with extra args, inheriting stdout/stderr
fn run_example(name: &str, extra_args: &[&str]) -> Result<()> {
    println!(
        "\n🔧 Running example `{}` with extra args: {:?}",
        name, extra_args
    );

    let current_dir = env::current_dir().context("Failed to get current directory")?;
    println!("📂 Current working directory: {}", current_dir.display());

    let mut cmd = Command::new("cargo");
    cmd.args(&["run", "--example", name, "--"]);
    cmd.args(extra_args);

    let cmdline = format!("{:?}", cmd);
    println!("📜 Full command: {}", cmdline);

    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let status = cmd
        .spawn()
        .with_context(|| format!("failed to spawn step `{}`", name))?
        .wait()
        .with_context(|| format!("failed to wait on step `{}`", name))?;

    if status.success() {
        println!("✅ Step `{}` completed successfully.", name);
    } else {
        println!("❌ Step `{}` FAILED with status: {}", name, status);
        anyhow::bail!("step `{}` failed with {}", name, status);
    }

    Ok(())
}
