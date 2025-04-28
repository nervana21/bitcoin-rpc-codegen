// examples/discover.rs
//
// Discover RPC methods and dump help output for each.

use anyhow::{bail, Context, Result};
use bitcoin_rpc_codegen::discover_methods;
use std::{
    env,
    fs::{create_dir_all, write},
    path::PathBuf,
    process::Command,
};

fn main() -> Result<()> {
    // --- Parse CLI args ---
    let mut args = env::args().skip(1);
    let mut bin_path = None;
    let mut version = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bin-path" => {
                bin_path = Some(PathBuf::from(
                    args.next().context("Expected path after --bin-path")?,
                ));
            }
            "--version" => {
                version = Some(
                    args.next()
                        .context("Expected version after --version (e.g., v29)")?,
                );
            }
            other => {
                bail!("Unknown argument: {}", other);
            }
        }
    }

    let bitcoind = bin_path.context("Missing required --bin-path argument")?;
    let ver = version.context("Missing required --version argument")?;
    if !bitcoind.exists() {
        bail!("bitcoind binary not found at: {}", bitcoind.display());
    }

    println!(
        "🚀 Discovering methods for version {} using {}",
        ver,
        bitcoind.display()
    );

    // Discover RPC method names
    let methods =
        discover_methods(&bitcoind).context("Failed to discover methods from bitcoin-cli")?;
    println!("Found {} methods", methods.len());

    // Prepare docs directory
    let docs_dir = PathBuf::from("resources").join(format!("{}_docs", ver));
    create_dir_all(&docs_dir)
        .with_context(|| format!("Failed to create docs directory {:?}", docs_dir))?;

    // Locate bitcoin-cli next to bitcoind
    let cli = bitcoind.parent().unwrap().join(if cfg!(windows) {
        "bitcoin-cli.exe"
    } else {
        "bitcoin-cli"
    });

    // Dump help output for each method
    for method in methods {
        print!("Dumping help for `{}`… ", method);
        let out = Command::new(&cli)
            .args(&["-regtest", "help", &method])
            .output();

        match out {
            Ok(o) if o.status.success() => {
                let text = String::from_utf8_lossy(&o.stdout);
                let path = docs_dir.join(format!("{}.txt", method));
                write(&path, text.as_ref())?;
                println!("ok");
            }
            Ok(o) => {
                eprintln!("error (exit {})", o.status.code().unwrap_or(-1));
            }
            Err(e) => {
                eprintln!("failed to spawn {}: {}", cli.display(), e);
            }
        }
    }

    println!("✅ Discovery complete — docs/{} populated.", ver);
    Ok(())
}
