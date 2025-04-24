use bitcoin_rpc_codegen::{generator, parser};
use std::path::PathBuf;
use std::{env, fs};

fn main() -> std::io::Result<()> {
    // find api_v29.json in the resources directory
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir.parent().unwrap().parent().unwrap();
    let api_file = workspace_root.join("resources").join("api_v29.json");
    let api_json = fs::read_to_string(&api_file)?;
    let methods = parser::parse_api_json(&api_json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 2. Build your list of versions
    let versions = vec!["v29".to_string()];

    // 3. Call the generators with that slice
    let out_dir = manifest_dir.join("target").join("generated");
    fs::create_dir_all(&out_dir)?;

    // Generate the mod.rs files
    generator::generate_types_mod_rs(&versions, &out_dir)?;
    generator::generate_client_mod_rs(&versions, &out_dir)?;

    // Generate the actual client and types files for v29
    let v29_dir = out_dir.join("v29");
    fs::create_dir_all(&v29_dir)?;

    // Generate client/mod.rs
    let client_dir = v29_dir.join("client");
    fs::create_dir_all(&client_dir)?;
    let mut client_mod = String::new();
    for method in &methods {
        client_mod.push_str(&generator::generate_client_macro(method, "v29"));
        client_mod.push_str("\n\n");
    }
    fs::write(client_dir.join("mod.rs"), client_mod)?;

    // Generate types/mod.rs
    let types_dir = v29_dir.join("types");
    fs::create_dir_all(&types_dir)?;
    let mut types_mod = String::from("use serde::{Deserialize, Serialize};\n\n");
    for method in &methods {
        if let Some(type_code) = generator::generate_return_type(method) {
            types_mod.push_str(&type_code);
            types_mod.push_str("\n\n");
        }
    }
    fs::write(types_dir.join("mod.rs"), types_mod)?;

    Ok(())
}
