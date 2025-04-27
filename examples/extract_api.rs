// examples/extract_api.rs

use anyhow::{Context, Result};
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiResult};
use bitcoin_rpc_codegen::Conf;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

fn main() -> Result<()> {
    // --- 🏛 Parse CLI args ---
    let mut args = env::args().skip(1);
    let version = match (args.next(), args.next()) {
        (Some(flag), Some(value)) if flag == "--version" => value,
        _ => {
            println!("Warning: No arguments provided, defaulting to v29");
            "v29".to_string()
        }
    };

    println!("Checking for docs directory...");
    let docs_dir_path = format!("resources/docs/{}_docs", version);
    let docs_dir = Path::new(&docs_dir_path);
    println!("Docs path: {}", docs_dir.display());

    if !docs_dir.exists() {
        println!("\u{274c} Docs directory does not exist!");
        anyhow::bail!(
            "❌ Missing {}/ — please run `cargo run --example discover -- --version {}` first.",
            docs_dir_path,
            version
        );
    } else {
        println!("Docs directory exists. Continuing...");
    }

    // --- 🔍 Setup bitcoind ---
    let home = env::var("HOME").context("Missing $HOME env var")?;
    let bin_path = PathBuf::from(&home).join(format!(
        "bitcoin-versions/{}/bitcoin-{}.0/bin/bitcoind",
        &version[1..],
        &version[1..]
    ));
    println!("Using bitcoind path: {}", bin_path.display());

    let mut conf = Conf::default();
    conf.wallet_name = "dummy";
    conf.view_stdout = false;
    conf.extra_args.push("-listen=0");

    let (mut child, _datadir, cookie, rpc_url) = spawn_node_with_custom_bin(&bin_path, &conf)?;
    let rpc = Client::new(&rpc_url, Auth::CookieFile(cookie))?;

    // --- 🔍 Fetch Help ---
    println!("Fetching top-level help...");
    let help_output: String = rpc.call("help", &[])?;

    let mut method_names = Vec::new();
    for line in help_output.lines() {
        if let Some(name) = line.split_whitespace().next() {
            if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                method_names.push(name.to_string());
            }
        }
    }
    println!("Found {} methods.", method_names.len());

    // --- 🔍 Dumping Docs ---
    println!("Dumping docs to: {}", docs_dir.display());
    for method in &method_names {
        match rpc.call::<String>("help", &[json!(method)]) {
            Ok(doc) => {
                let path = docs_dir.join(format!("{method}.txt"));
                println!("Saving: {}", path.display());
                fs::write(&path, &doc)
                    .with_context(|| format!("Failed to write {}", path.display()))?;
            }
            Err(e) => {
                eprintln!("⚠️  could not dump `{method}`: {e}");
            }
        }
    }

    // --- 📊 Build JSON schema ---
    println!("Parsing method docs...");
    let mut commands = Map::new();
    let arg_re = Regex::new(r#"^\s*\d+\.\s+\"?([^\"\s]+)\"?\s*\(([^)]+)\)\s*(.*)$"#).unwrap();

    for method in &method_names {
        let doc_path = docs_dir.join(format!("{method}.txt"));
        let doc = fs::read_to_string(&doc_path).with_context(|| {
            format!(
                "Missing help text for `{method}` at `{}`",
                doc_path.display()
            )
        })?;

        let description = extract_description(&doc);
        let arguments = infer_arguments(&doc, &arg_re);
        let results = infer_results(&doc);

        let entry = json!({
            "name": method,
            "description": description,
            "arguments": arguments,
            "results": results,
        });
        commands.insert(method.clone(), json!([entry]));
    }

    println!("Finalizing JSON output...");
    let schema_dir = Path::new("resources/schemas");
    fs::create_dir_all(schema_dir)?;

    let out = serde_json::to_string_pretty(&json!({ "commands": commands }))?;
    let final_path = schema_dir.join(format!("api_{}.json", version));
    println!("Writing schema to: {}", final_path.display());
    let mut file = File::create(final_path)?;
    writeln!(file, "{out}")?;

    // --- 🛁 Clean Shutdown ---
    println!("Stopping bitcoind...");
    let _ = rpc.call::<Value>("stop", &[]);
    let _ = child.wait();

    println!(
        "Done! Successfully extracted {} methods.",
        method_names.len()
    );
    Ok(())
}

/// Skip the first signature line, then grab everything until the next section.
fn extract_description(doc: &str) -> String {
    doc.lines()
        .skip(1)
        .skip_while(|l| l.trim().is_empty())
        .take_while(|l| {
            !l.starts_with("Arguments:")
                && !l.starts_with("Result:")
                && !l.starts_with("Returns:")
                && !l.starts_with("Examples:")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Pull out numbered `.1 (name) (type) description…` lines
fn infer_arguments(doc: &str, re: &Regex) -> Vec<ApiArgument> {
    let mut args = Vec::new();
    let mut in_args = false;
    for line in doc.lines() {
        let t = line.trim();
        if t.starts_with("Arguments:") {
            in_args = true;
            continue;
        }
        if in_args {
            if t.is_empty() || t.ends_with(':') {
                break;
            }
            if let Some(c) = re.captures(line) {
                let name = c[1].to_string();
                let typ = c[2].to_lowercase();
                let desc = c[3].trim().to_string();
                let optional = typ.contains("optional");
                args.push(ApiArgument {
                    names: vec![name],
                    type_: typ,
                    optional,
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

        let result = ApiResult {
            type_: typ.to_string(),
            description: desc.clone(),
            key_name,
            inner: Vec::new(),
        };

        while depth < stack.last().unwrap().0 {
            let (_, mut inner) = stack.pop().unwrap();
            if let Some((_, parent)) = stack.last_mut() {
                parent.last_mut().unwrap().inner.append(&mut inner);
            }
        }

        stack.last_mut().unwrap().1.push(result);
        stack.push((depth, Vec::new()));
    }

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
    use std::{thread::sleep, time::Duration};

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
