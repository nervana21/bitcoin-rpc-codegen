// examples/extract_api_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiMethod, ApiResult};
use bitcoin_rpc_codegen::Conf;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use regex::Regex;
use serde::Serialize;
use serde_json::json;
use std::{
    collections::BTreeMap,
    fs,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    process::Child,
};

#[derive(Serialize)]
struct Schema {
    commands: BTreeMap<String, Vec<ApiMethod>>,
}

fn main() -> Result<()> {
    // 1) Launch a regtest node
    let home = std::env::var("HOME")?;
    let bin_path = PathBuf::from(&home).join("bitcoin-versions/v29/bitcoin-29.0/bin/bitcoind");
    let mut conf = Conf::default();
    conf.wallet_name = "dummy";
    conf.view_stdout = false;
    conf.extra_args.push("-listen=0");
    let (mut child, _datadir, cookie, rpc_url) = spawn_node_with_custom_bin(&bin_path, &conf)?;
    let rpc = Client::new(&rpc_url, Auth::CookieFile(cookie))?;

    // 2) Discover all method names
    println!("📜 Fetching top-level help…");
    let help_output: String = rpc.call("help", &[])?;
    let mut method_names = Vec::new();
    for line in help_output.lines() {
        if let Some(name) = line.split_whitespace().next() {
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                method_names.push(name.to_string());
            }
        }
    }
    println!("🔍 Found {} methods", method_names.len());

    // 3) For each method, fetch its help and infer schema
    let mut commands = BTreeMap::new();
    for name in method_names {
        let doc: String = rpc.call("help", &[json!(name.clone())])?;

        let description = extract_description(&doc);
        let arguments = infer_arguments(&doc);
        let results = infer_results(&doc);

        let m = ApiMethod {
            name: name.clone(),
            description,
            arguments,
            results,
        };
        commands.insert(name, vec![m]);
    }

    // 4) Serialize into resources/api_v29.json with the proper wrapper
    let schema = Schema { commands };
    let out = serde_json::to_string_pretty(&schema)?;
    let mut file = File::create(Path::new("resources/api_v29.json"))?;
    writeln!(file, "{}", out)?;
    println!(
        "✅ Wrote {} methods to resources/api_v29.json",
        schema.commands.len()
    );

    // 5) Shutdown
    let _ = rpc.call::<serde_json::Value>("stop", &[]);
    let _ = child.wait();
    Ok(())
}

/// Skip the signature line, capture up to the next section header.
fn extract_description(doc: &str) -> String {
    doc.lines()
        .skip(1)
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| {
            let t = l.trim_start();
            !t.starts_with("Arguments:")
                && !t.starts_with("Result")
                && !t.starts_with("Returns")
                && !t.starts_with("Examples:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn infer_arguments(doc: &str) -> Vec<ApiArgument> {
    let mut args = Vec::new();
    let mut in_args = false;
    let name_re = Regex::new(r#"^(\d+)\.\s+"?([a-zA-Z0-9_]+)"?\s+\(([^)]+)\)\s+(.*)$"#).unwrap();

    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Arguments:") {
            in_args = true;
            continue;
        }
        if in_args
            && (t.starts_with("Result") || t.starts_with("Returns") || t.starts_with("Examples:"))
        {
            break;
        }
        if in_args {
            if let Some(caps) = name_re.captures(t) {
                let name = caps[2].to_string();
                let typ = caps[3].to_lowercase();
                let desc = caps[4].to_string();
                args.push(ApiArgument {
                    names: vec![name],
                    type_: typ.clone(),
                    optional: typ.contains("optional"),
                    description: desc,
                });
            }
        }
    }

    args
}

fn infer_results(doc: &str) -> Vec<ApiResult> {
    let mut in_res = false;
    let mut stack: Vec<(usize, Vec<ApiResult>)> = vec![(0, Vec::new())];
    let key_re = Regex::new(r#"^"([^"]+)":\s*(.*)$"#).unwrap();

    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Result") || t.starts_with("Returns") {
            in_res = true;
            continue;
        }
        if in_res
            && (t.starts_with("Arguments:")
                || t.starts_with("Examples:")
                || t.chars().next().map_or(false, |c| c.is_digit(10)))
        {
            break;
        }
        if !in_res || t.is_empty() {
            continue;
        }

        let depth = line.chars().take_while(|c| c.is_whitespace()).count();
        let mut typ = "string";
        let mut desc = t.to_string();
        let mut key_name = String::new();

        if let Some(cap) = key_re.captures(t) {
            key_name = cap[1].to_string();
            desc = cap[2].to_string();
        }

        if desc.contains("(boolean)") {
            typ = "boolean";
        } else if desc.contains("(numeric)") {
            typ = "number";
        } else if desc.contains("(json object)") {
            typ = "object";
        } else if desc.contains("(json null)") {
            typ = "none";
        }

        let mut result = ApiResult {
            type_: typ.to_string(),
            description: desc.clone(),
            key_name,
            inner: Vec::new(),
        };

        // Pop back up for shallower depth
        while depth < stack.last().unwrap().0 {
            let (_, mut inner) = stack.pop().unwrap();
            if let Some((_, parent)) = stack.last_mut() {
                parent.last_mut().unwrap().inner.append(&mut inner);
            }
        }

        stack.last_mut().unwrap().1.push(result);
        stack.push((depth, Vec::new()));
    }

    // unwind
    while stack.len() > 1 {
        let (_, mut inner) = stack.pop().unwrap();
        if let Some((_, parent)) = stack.last_mut() {
            parent.last_mut().unwrap().inner.append(&mut inner);
        }
    }

    let final_res = stack.pop().unwrap().1;
    if final_res.is_empty() {
        vec![ApiResult {
            type_: "none".into(),
            description: String::new(),
            key_name: String::new(),
            inner: Vec::new(),
        }]
    } else {
        final_res
    }
}

fn spawn_node_with_custom_bin(
    bin_path: &Path,
    conf: &Conf<'_>,
) -> Result<(Child, tempfile::TempDir, PathBuf, String)> {
    use bitcoin_rpc_codegen::regtest::{get_available_port, wait_for_rpc_ready};
    use std::{process::Command, process::Stdio, thread::sleep, time::Duration};

    let mut last_err = None;
    for attempt in 1..=conf.attempts {
        let datadir = tempfile::TempDir::new()?;
        let port = get_available_port()?;
        let url = format!("http://127.0.0.1:{}", port);
        let cookie = datadir.path().join("regtest").join(".cookie");

        let mut cmd = Command::new(bin_path);
        cmd.args(&[
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
                }
            }
        }
    }
    Err(last_err.unwrap().into())
}
