// examples/sanity_check.rs

use anyhow::Result;
use bitcoin_rpc_codegen::Conf;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde_json::json;
use std::{collections::BTreeSet, fs::write, path::PathBuf};

fn main() -> Result<()> {
    // IDENTICAL start to extract_api_v29.rs
    let home = std::env::var("HOME")?;
    let bin_path = PathBuf::from(&home).join("bitcoin-versions/v29/bitcoin-29.0/bin/bitcoind");

    let mut conf = Conf::default();
    conf.wallet_name = "dummy";
    conf.view_stdout = false;
    conf.extra_args.push("-listen=0");

    let (mut child, _datadir, cookie, rpc_url) = spawn_node_with_custom_bin(&bin_path, &conf)?;
    let rpc = Client::new(&rpc_url, Auth::CookieFile(cookie))?;

    println!("🚀 Hello, world!");
    println!("📜 Fetching full method list from `help`…");

    let info = rpc.get_network_info()?;
    println!("  version     = {}", info.version);
    println!("  subversion  = {}", info.subversion);
    println!("  protocol    = {}", info.protocol_version);

    let help_output: String = rpc.call("help", &[])?;
    let mut method_names = BTreeSet::new();

    for line in help_output.lines() {
        if let Some(name) = line.split_whitespace().next() {
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                method_names.insert(name.to_string());
            }
        }
    }

    println!("✅ Found {} RPC methods", method_names.len());

    let output_dir = PathBuf::from("resources/v29_docs");
    std::fs::create_dir_all(&output_dir)?;

    for method in &method_names {
        match rpc.call::<String>("help", &[json!(method)]) {
            Ok(doc) => {
                println!("   • {method}: ok ({} bytes)", doc.len());

                // NEW: Dump help text to disk
                let file_path = output_dir.join(format!("{method}.txt"));
                write(&file_path, doc)?;
            }
            Err(e) => {
                println!("   • {method}: ❌ ERROR — {e}");
            }
        }
    }

    let _ = rpc.call::<serde_json::Value>("stop", &[]); // clean shutdown
    let _ = child.wait();

    Ok(())
}

fn spawn_node_with_custom_bin(
    bin_path: &std::path::Path,
    conf: &Conf<'_>,
) -> Result<(
    std::process::Child,
    tempfile::TempDir,
    std::path::PathBuf,
    String,
)> {
    use bitcoin_rpc_codegen::regtest::{get_available_port, wait_for_rpc_ready};
    use std::process::{Command, Stdio};
    use std::{thread::sleep, time::Duration};

    let mut last_err = None;

    for attempt in 1..=conf.attempts {
        let datadir = tempfile::TempDir::new()?;
        let port = get_available_port()?;
        let url = format!("http://127.0.0.1:{}", port);
        let cookie = datadir.path().join("regtest").join(".cookie");

        let mut cmd = Command::new(bin_path);
        cmd.args([
            "-regtest",
            &format!("-datadir={}", datadir.path().display()),
            &format!("-rpcport={}", port),
            &format!("-rpcbind=127.0.0.1:{}", port),
            "-rpcallowip=127.0.0.1",
            "-fallbackfee=0.0002",
        ]);
        if conf.enable_txindex {
            cmd.arg("-txindex");
        }
        cmd.args(&conf.extra_args);
        if conf.view_stdout {
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());
        } else {
            cmd.stdout(Stdio::null()).stderr(Stdio::null());
        }

        let mut child = cmd.spawn()?;
        match wait_for_rpc_ready(&url, &cookie, &mut child) {
            Ok(()) => return Ok((child, datadir, cookie, url)),
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                last_err = Some(e);
                if attempt < conf.attempts {
                    sleep(Duration::from_millis(200));
                    continue;
                }
            }
        }
    }

    Err(last_err.unwrap().into())
}
