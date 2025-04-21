// examples/extract_api_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiResult};
use bitcoin_rpc_codegen::Conf;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let home = std::env::var("HOME")?;
    let bin_path = PathBuf::from(&home).join("bitcoin-versions/v29/bitcoin-29.0/bin/bitcoind");

    let mut conf = Conf::default();
    conf.wallet_name = "dummy";
    conf.view_stdout = false;
    conf.extra_args.push("-listen=0");

    let (mut child, _datadir, cookie, rpc_url) = spawn_node_with_custom_bin(&bin_path, &conf)?;
    let rpc = Client::new(&rpc_url, Auth::CookieFile(cookie))?;

    println!("📜 Fetching top-level help...");
    let help_output: String = rpc.call("help", &[])?;

    let mut method_names = vec![];
    for line in help_output.lines() {
        if let Some(name) = line.split_whitespace().next() {
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                method_names.push(name.to_string());
            }
        }
    }

    println!(
        "🔍 Found {} methods, fetching individual help...",
        method_names.len()
    );

    let mut commands = Map::new();
    for method in method_names {
        let doc: String = rpc.call("help", &[json!(method)])?;

        let description = extract_description(&doc);
        let examples = extract_examples(&doc);
        let arguments = infer_arguments(&doc);
        let results = infer_results(&doc);

        let method_obj = json!({
            "name": method.clone(),
            "description": description,
            "arguments": arguments,
            "results": results,
            "examples": examples,
        });

        commands.insert(method.clone(), json!([method_obj]));
    }

    let output_path = Path::new("resources/api_v29.json");
    let mut file = File::create(output_path)?;
    writeln!(
        file,
        "{}",
        serde_json::to_string_pretty(&json!({ "commands": commands }))?
    )?;

    println!(
        "✅ Wrote {} methods to {}",
        commands.len(),
        output_path.display()
    );

    let _ = rpc.call::<Value>("stop", &[]); // attempt clean shutdown
    let _ = child.wait();

    Ok(())
}

fn extract_description(doc: &str) -> String {
    doc.lines()
        .take_while(|l| {
            !l.trim_start().starts_with("Arguments:")
                && !l.trim_start().starts_with("Result:")
                && !l.trim_start().starts_with("Returns:")
                && !l.trim_start().starts_with("Examples:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_examples(doc: &str) -> String {
    let mut lines = doc
        .lines()
        .skip_while(|l| !l.trim_start().starts_with("Examples:"));
    lines.next(); // skip "Examples:"
    lines.collect::<Vec<_>>().join("\n")
}

fn infer_arguments(doc: &str) -> Vec<ApiArgument> {
    let mut capture = false;
    let mut args = vec![];

    let name_re = Regex::new(r#"^(\d+)\.\s+"?([a-zA-Z0-9_]+)"?\s+\(([^)]+)\)\s+(.*)$"#).unwrap();

    for line in doc.lines() {
        let line = line.trim();

        if line.starts_with("Arguments:") {
            capture = true;
            continue;
        }

        if line.starts_with("Result")
            || line.starts_with("Returns")
            || line.starts_with("Examples:")
            || line.starts_with("1.")
        {
            if capture {
                break;
            }
        }

        if capture {
            if let Some(caps) = name_re.captures(line) {
                let name = caps[2].to_string();
                let typ = caps[3].to_lowercase();
                let desc = caps[4].to_string();

                let mapped_type = if typ.contains("string") {
                    "string"
                } else if typ.contains("hex") {
                    "hex"
                } else if typ.contains("bool") {
                    "boolean"
                } else if typ.contains("number") || typ.contains("numeric") {
                    "number"
                } else if typ.contains("array") {
                    "array"
                } else if typ.contains("object") {
                    "object"
                } else {
                    "string"
                };

                args.push(ApiArgument {
                    names: vec![name],
                    type_: mapped_type.to_string(),
                    optional: typ.contains("optional"),
                    description: desc,
                });
            }
        }
    }

    args
}

fn infer_results(doc: &str) -> Vec<ApiResult> {
    let mut capture = false;
    let mut stack: Vec<(usize, Vec<ApiResult>)> = vec![(0, vec![])];
    let key_re = Regex::new(r#"^"([^"]+)"\s*:\s*(.*)$"#).unwrap();

    for line in doc.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Result")
            || trimmed.starts_with("Returns")
            || trimmed.starts_with("Result =")
        {
            capture = true;
            continue;
        }

        if trimmed.starts_with("Arguments:")
            || trimmed.starts_with("Examples:")
            || trimmed.starts_with("1.")
            || trimmed.starts_with("0.")
        {
            break;
        }

        if !capture || trimmed.is_empty() {
            continue;
        }

        let depth = line.chars().take_while(|c| c.is_whitespace()).count();
        let mut type_ = "string";
        let mut desc = trimmed.to_string();
        let mut key_name = String::new();

        if let Some(cap) = key_re.captures(trimmed) {
            key_name = cap[1].to_string().trim().to_string();
            desc = cap[2].to_string().trim().to_string();
        }

        if desc.contains("(boolean)") {
            type_ = "boolean";
            desc = desc.replace("(boolean)", "").trim().to_string();
        } else if desc.contains("(string)") {
            type_ = "string";
            desc = desc.replace("(string)", "").trim().to_string();
        } else if desc.contains("(hex)") {
            type_ = "hex";
            desc = desc.replace("(hex)", "").trim().to_string();
        } else if desc.contains("(numeric)") {
            type_ = "number";
            desc = desc.replace("(numeric)", "").trim().to_string();
        } else if desc.contains("(json object)") {
            type_ = "object";
            desc = desc.replace("(json object)", "").trim().to_string();
        } else if desc.contains("(json null)") {
            type_ = "none";
            desc = desc.replace("(json null)", "").trim().to_string();
        }

        let result = ApiResult {
            type_: type_.to_string(),
            description: desc,
            key_name,
            inner: vec![],
        };

        while depth < stack.last().unwrap().0 {
            let (_child_depth, mut inner_results) = stack.pop().unwrap();
            if let Some((_parent_depth, parent)) = stack.last_mut() {
                if let Some(last) = parent.last_mut() {
                    last.inner.append(&mut inner_results);
                }
            }
        }

        stack.last_mut().unwrap().1.push(result);
        stack.push((depth, vec![]));
    }

    while stack.len() > 1 {
        let (_, mut inner_results) = stack.pop().unwrap();
        if let Some((_parent_depth, parent)) = stack.last_mut() {
            if let Some(last) = parent.last_mut() {
                last.inner.append(&mut inner_results);
            }
        }
    }

    let final_results = stack.pop().unwrap().1;
    if final_results.is_empty() {
        vec![ApiResult {
            type_: "string".to_string(),
            description: "".to_string(),
            key_name: "".to_string(),
            inner: vec![],
        }]
    } else {
        final_results
    }
}

fn spawn_node_with_custom_bin(
    bin_path: &Path,
    conf: &Conf<'_>,
) -> Result<(std::process::Child, tempfile::TempDir, PathBuf, String)> {
    use bitcoin_rpc_codegen::regtest::{get_available_port, wait_for_rpc_ready};
    use std::{
        process::{Command, Stdio},
        thread::sleep,
        time::Duration,
    };

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
