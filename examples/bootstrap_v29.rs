// examples/bootstrap_v29.rs
//
// Runs the full v29 pipeline:
//  1. discover  ‣ dump help texts
//  2. extract   ‣ build raw JSON schema
//  3. regenerate‣ canonicalize schema
//  4. validate  ‣ zero-arg roundtrip
//  5. generate  ‣ Rust client + types
//
// Invocation:
//   cargo run --example bootstrap_v29

use anyhow::{Context, Result};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    // You can tweak VERSION or BIN_PATH if you ever want to bump to core-30, etc.
    const VERSION: &str = "v29";
    let bin_path = format!(
        "{home}/bitcoin-versions/{ver}/bitcoin-{ver}.0/bin/bitcoind",
        home = std::env::var("HOME").unwrap(),
        ver = &VERSION[1..]
    );

    // List of (example_name, extra_args…) to run in order:
    let steps = &[
        ("discover", &["--bin-path", &bin_path][..]),
        ("extract_api_v29", &[][..]),
        ("regenerate_schema_v29", &[][..]),
        ("validate_roundtrip_v29", &[][..]),
        ("generate_v29", &[][..]),
    ];

    for (name, extra) in steps {
        println!("\n=== STEP `{}` ===", name);
        // spawn `cargo run --example <name> -- <extra_args...>`
        let mut cmd = Command::new("cargo");
        cmd.args(&["run", "--example", name, "--"]);
        cmd.args(*extra);
        // inherit stdout/stderr so you see everything
        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        let status = cmd
            .spawn()
            .with_context(|| format!("failed to spawn step `{}`", name))?
            .wait()
            .with_context(|| format!("failed to wait on step `{}`", name))?;
        if !status.success() {
            anyhow::bail!("step `{}` failed with {}", name, status);
        }
    }

    println!("\n✅ bootstrap_v29 complete — all steps ran successfully and deterministically!");
    Ok(())
}
