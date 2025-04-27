// examples/lib.rs

use anyhow::{Context, Result};
use std::process::{Command, Stdio};

/// Helper to run a cargo example with extra args, inheriting stdout/stderr
pub fn run_example(name: &str, extra_args: &[&str]) -> Result<()> {
    println!(
        "\n🔧 Running example `{}` with extra args: {:?}",
        name, extra_args
    );

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

fn main() {
    // This binary is not meant to be run directly.
    println!("This is a library module, not a runnable example.");
}
