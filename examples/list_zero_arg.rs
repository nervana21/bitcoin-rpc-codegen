// examples/list_zero_arg.rs

use bitcoin_rpc_codegen::parser::parse_api_json;
use std::fs;

fn main() {
    let schema_src = fs::read_to_string("resources/api_v29.json").expect("read schema");
    let methods = parse_api_json(&schema_src).expect("parse schema");

    let zero_arg: Vec<_> = methods
        .into_iter()
        .filter(|m| m.arguments.is_empty())
        .map(|m| m.name)
        .collect();

    println!("Zero-arg RPCs (in schema order):");
    for name in &zero_arg {
        println!("  {}", name);
    }
    println!("\nLast zero-arg RPC is: {}", zero_arg.last().unwrap());
}
