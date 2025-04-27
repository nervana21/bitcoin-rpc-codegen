// examples/bootstrap_version.rs
//
// Full Bitcoin Core version bootstrap pipeline.
//  0. (Optional) verify_all_methods to populate feedback
//  1. discover               ‣ dump help texts (only if missing or forced)
//  2. extract_api            ‣ build raw JSON schema
//  3. regenerate_schema      ‣ canonicalize schema
//  4. validate_roundtrip     ‣ zero-arg roundtrip sanity check
//  5. generate               ‣ emit Rust client + types
//
// Invocation:
//   cargo run --example bootstrap_version -- v29
//   cargo run --example bootstrap_version -- v30
//   (add --force to re-run discovery/feedback)

use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let mut args = env::args().skip(1); // Skip program name

    let version = args.next().context("Missing version argument (e.g., v29)")?;
    let force = args.any(|arg| arg == "--force");

    let home = env::var("HOME").context("Failed to get $HOME env var")?;
    let bin_path = format!(
        "{}/bitcoin-versions/v{}/bitcoin-{}.0/bin/bitcoind",
        home,
        &version[1..],
        &version[1..]
    );

    println!("🚀 Starting bootstrap for version: {}", version);
    if force {
        println!("⚡ Force mode enabled — will re-run discovery and feedback steps.");
    }

    // --- 📂 Step 0: Ensure feedback/ is populated ---
    if force || !Path::new("feedback").exists() {
        println!("📂 Running `verify_all_methods` to (re)generate feedback...");
        run_example("verify_all_methods", &["--version", &version])?;
        println!("✅ `feedback/` generated successfully.");
    } else {
        println!("📂 `feedback/` directory already exists. Skipping feedback generation.");
    }

    // --- 📂 Step 0b: Ensure version_docs/ exists ---
    let index_path = format!("resources/{}_docs/index.txt", version);
    if force || !Path::new(&index_path).exists() {
        println!("📂 Running `discover` to (re)generate method docs...");
        run_example("discover", &["--bin-path", &bin_path, "--version", &version])?;
        println!("✅ `resources/{}_docs/` generated successfully.", version);
    } else {
        println!("📂 `{}` already exists. Skipping discovery.", index_path);
    }

    // --- 🚀 Main pipeline steps ---
    let steps = &[
        ("extract_api", &["--version", &version]),
        ("regenerate_schema", &["--version", &version]),
        ("validate_roundtrip", &["--version", &version]),
        ("generate", &["--version", &version]),
    ];

    for (name, extra) in steps {
        println!("\n=== STEP `{}` ===", name);
        run_example(name, *extra)?;
    }

    println!("\n✅ bootstrap_version complete — all steps ran successfully and deterministically!");
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

    cmd.stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

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
