// examples/validate_api_v29.rs

use anyhow::Result;
use bitcoin_rpc_codegen::parser::{parse_api_json, ApiMethod};
use std::{collections::BTreeSet, fs, path::Path};

fn main() -> Result<()> {
    println!("🔍 Validating parsed api_v29.json…");

    let api_json = fs::read_to_string("resources/api_v29.json")?;
    let methods: Vec<ApiMethod> = parse_api_json(&api_json)?;

    // 1. Check method count matches docs directory
    let docs_dir = Path::new("resources/v29_docs");
    let mut expected_methods = BTreeSet::new();
    for entry in fs::read_dir(docs_dir)? {
        let path = entry?.path();
        if let Some(stem) = path.file_stem() {
            expected_methods.insert(stem.to_string_lossy().to_string());
        }
    }

    let parsed_method_names: BTreeSet<String> = methods.iter().map(|m| m.name.clone()).collect();

    assert_eq!(
        parsed_method_names, expected_methods,
        "Mismatch between doc files and parsed JSON method names"
    );

    // 2. Validate each method's required fields
    for method in &methods {
        assert!(
            !method.name.is_empty(),
            "Missing method name in parsed schema"
        );
        assert!(
            !method.description.trim().is_empty(),
            "Missing description for method `{}`",
            method.name
        );

        for arg in &method.arguments {
            assert!(
                !arg.names.is_empty(),
                "Missing argument name in method `{}`",
                method.name
            );
            assert!(
                !arg.type_.trim().is_empty(),
                "Missing type for argument `{}` in method `{}`",
                arg.names[0],
                method.name
            );
        }

        for res in &method.results {
            assert!(
                !res.type_.trim().is_empty(),
                "Missing result type in method `{}`",
                method.name
            );
        }
    }

    println!("✅ api_v29.json passed all validation checks.");
    Ok(())
}
