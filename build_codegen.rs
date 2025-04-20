// build_codegen.rs

#[path = "src/parser/mod.rs"]
mod parser;

#[path = "src/generator/mod.rs"]
mod generator;

use generator::{generate_client_macro, generate_mod_rs, SUPPORTED_VERSIONS};
use parser::{parse_api_json, ApiMethod};

use anyhow::Result;
use std::{collections::HashMap, fs, path::Path};

use crate::generator::{generate_client_macro, generate_mod_rs, generate_return_type};
use crate::parser::{parse_api_json, ApiMethod};

pub fn run_codegen() -> Result<()> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let api_path = Path::new(manifest_dir).join("resources").join("api.json");
    if !api_path.exists() {
        return Err(anyhow::anyhow!(
            "API JSON file not found at: {:?}",
            api_path
        ));
    }
    println!("run_codegen: Using API JSON file at: {:?}", api_path);

    let api_json = fs::read_to_string(api_path)?;
    let methods = parse_api_json(&api_json)?;

    let out_dir = std::env::var("OUT_DIR")?;
    if fs::metadata(&out_dir).is_ok() {
        fs::remove_dir_all(&out_dir)?;
    }
    fs::create_dir_all(&out_dir)?;

    for version in SUPPORTED_VERSIONS {
        generate_version_code(version, &methods, &out_dir)?;
    }

    generate_mod_rs(&out_dir, crate::generator::SUPPORTED_VERSIONS)?;
    println!(
        "run_codegen: Code generation complete. Files saved in {:?}",
        out_dir
    );
    Ok(())
}

fn generate_version_code(version: &str, methods: &[ApiMethod], out_dir: &str) -> Result<()> {
    let root_dir = Path::new(out_dir);
    let client_dir = root_dir.join("client/src").join(version);
    let types_dir = root_dir.join("types/src").join(version);

    for dir in &[&client_dir, &types_dir] {
        if dir.exists() {
            fs::remove_dir_all(dir)?;
        }
        fs::create_dir_all(dir)?;
    }

    let type_imports = r#"use serde::{Deserialize, Serialize};
"#;

    let mut methods_by_category: HashMap<String, Vec<&ApiMethod>> = HashMap::new();
    for method in methods {
        methods_by_category
            .entry(method.category.clone())
            .or_default()
            .push(method);
    }

    for (category, category_methods) in &methods_by_category {
        let mut client_code = String::new();
        for method in category_methods {
            client_code.push_str(&generate_client_macro(method, version));
            client_code.push_str("\n\n");
        }
        let client_path = format!("{}/{}.rs", client_dir.display(), category);
        fs::write(&client_path, client_code)?;

        let mut types_code = String::new();
        types_code.push_str(type_imports);
        for method in category_methods {
            if let Some(type_code) = generate_return_type(&method) {
                types_code.push_str(&type_code);
                types_code.push_str("\n\n");
            }
        }
        let types_path = format!("{}/{}.rs", types_dir.display(), category);
        fs::write(types_path, types_code)?;
    }

    Ok(())
}
