use anyhow::Result;
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiArgument, ApiMethod};
use bitcoin_rpc_codegen::{Conf, RegtestClient};
use serde_json::{json, Map, Value};
use std::fs;

fn default_params(args: &[ApiArgument], blockhash: &str, txid: &str) -> Vec<Value> {
    args.iter()
        .map(|arg| {
            let name = &arg.names[0];
            match () {
                _ if name.contains("blockhash") => Value::String(blockhash.into()),
                _ if name.contains("txid") => Value::String(txid.into()),
                _ if name.contains("address")
                    || name.contains("dest")
                    || name.contains("pubkey") =>
                {
                    Value::String("bcrt1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".into())
                }
                _ if name.contains("label") => Value::String("".into()),
                _ => match arg.type_.as_str() {
                    "string" | "hex" => Value::String("".into()),
                    "number" => Value::Number(0.into()),
                    "boolean" => Value::Bool(false),
                    "array" => Value::Array(vec![]),
                    "object" => Value::Object(Map::new()),
                    _ => Value::Null,
                },
            }
        })
        .collect()
}

fn main() -> Result<()> {
    println!("🔍 Loading RPC schema...");
    let schema_src = fs::read_to_string("resources/api_v29.json")?;
    let methods: Vec<ApiMethod> = parse_api_json(&schema_src)?;
    println!("✅ Loaded schema with {} methods", methods.len());

    let mut zero_arg_methods: Vec<_> = methods.iter().filter(|m| m.arguments.is_empty()).collect();
    let mut param_methods: Vec<_> = methods.iter().filter(|m| !m.arguments.is_empty()).collect();

    zero_arg_methods.sort_by_key(|m| &m.name);
    param_methods.sort_by_key(|m| &m.name);

    println!("🚀 Starting regtest node...");
    let mut conf = Conf::default();
    conf.extra_args.push("-fallbackfee=0.0002");
    let rt = RegtestClient::new_with_conf(&conf)?;
    let client = &rt.client;

    println!("🔑 Generating wallet-owned mining address...");
    let mining_address: String = client
        .call_json("getnewaddress", &[])?
        .as_str()
        .unwrap()
        .to_string();
    println!("✅ Generated mining address: {}", mining_address);

    println!("⛏️ Mining initial blocks...");
    client.call_json("generatetoaddress", &[json!(101), json!(mining_address)])?;

    let blockhash: String = client
        .call_json("getblockhash", &[json!(1)])?
        .as_str()
        .unwrap()
        .into();
    let block: Value = client.call_json("getblock", &[json!(blockhash.clone())])?;
    let txid = block["tx"][0].as_str().unwrap().to_string();

    println!("📌 Using blockhash: {}", blockhash);
    println!("📌 Using txid: {}", txid);

    println!("\n🌟 Verifying zero-arg methods...");
    for method in &zero_arg_methods {
        if method.name == "stop" {
            continue; // explicitly skip `stop` here
        }
        println!("🔸 Calling `{}`...", method.name);
        match client.call_json(&method.name, &[]) {
            Ok(resp) => println!("✅ `{}` succeeded: {}", method.name, resp),
            Err(e) => println!("⚠️ `{}` RPC error: {}", method.name, e),
        }
    }

    println!("\n🌟 Verifying methods with parameters...");
    for method in &param_methods {
        println!("🔹 Calling `{}` with generated parameters...", method.name);
        let params = default_params(&method.arguments, &blockhash, &txid);
        println!("   Params: {:?}", params);

        match client.call_json(&method.name, &params) {
            Ok(resp) => println!("✅ `{}` succeeded: {}", method.name, resp),
            Err(e) => println!("⚠️ `{}` RPC error: {}", method.name, e),
        }
    }

    println!("\n🛑 Calling `stop` at the end...");
    match client.call_json("stop", &[]) {
        Ok(resp) => println!("✅ `stop` succeeded: {}", resp),
        Err(e) => println!("⚠️ `stop` RPC error: {}", e),
    }

    println!("✅ Node stopping... Verifying shutdown...");
    match client.call_json("getblockcount", &[]) {
        Ok(_) => println!("❌ Node still running after stop command!"),
        Err(e) => {
            let e_str = format!("{}", e);
            if e_str.contains("connection refused")
                || e_str.contains("transport error")
                || e_str.contains("unexpected HTTP code: 503")
            {
                println!("✅ Node shutdown confirmed (transport error as expected).");
            } else {
                println!("⚠️ Unexpected shutdown behavior: {}", e_str);
            }
        }
    }

    Ok(())
}
