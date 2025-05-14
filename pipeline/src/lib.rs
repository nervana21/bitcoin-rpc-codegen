//! High‑level pipeline that ties together discovery / parsing,
//! schema normalisation, and code generation.

//! Pipeline crate: wire discovery / schema / codegen together.

use anyhow::Result;
use codegen::{
    rpc_method_discovery, write_generated, BasicCodeGenerator, CodeGenerator,
    TransportCodeGenerator, TypesCodeGenerator,
};
use parser::{DefaultHelpParser, HelpParser, MethodHelp};
use rpc_api::parse_api_json;
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};

use std::fs;
use std::path::PathBuf;

/* --------------------------------------------------------------------- */
/*  Public entry: run() – file‑driven                                    */
/* --------------------------------------------------------------------- */

/// Generate code from either a `help.txt` dump or a structured `api.json`.
pub fn run(input_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    tracing::info!(?input_path, ?out_dir, "Running file‑driven pipeline");
    // Prepare output tree ---------------------------------------------------
    fs::create_dir_all(out_dir)?;
    let transport_dir = out_dir.join("transport");
    let client_dir = out_dir.join("client");
    let types_dir = out_dir.join("types");
    fs::create_dir_all(&transport_dir)?;
    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    // Parse input -----------------------------------------------------------
    let (norm, src_description) = if input_path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        // ---- Structured schema path (no normalisation needed) ----
        let json = fs::read_to_string(input_path)?;
        (parse_api_json(&json)?, "structured JSON")
    } else {
        // ---- help.txt path → needs normalisation ----
        let help = fs::read_to_string(input_path)?;
        let raw = DefaultHelpParser.parse(&help)?;
        (DefaultSchemaNormalizer.normalize(&raw)?, "help dump")
    };

    DefaultSchemaValidator.validate(&norm)?;
    tracing::info!("Loaded {} methods from {}", norm.len(), src_description);

    // Generate transport modules -------------------------------------------
    let transport_files = TransportCodeGenerator.generate(&norm);
    write_generated(&transport_dir, &transport_files)?;

    // Generate response‑type structs ---------------------------------------
    let type_files = TypesCodeGenerator.generate(&norm);
    println!("TypesCodeGenerator produced {} files", type_files.len());
    for (name, _) in &type_files {
        println!("Type file: {}", name);
    }
    write_generated(&types_dir, &type_files)?;

    // Generate client stubs (placeholder) -----------------------------------
    let client_files = BasicCodeGenerator.generate(&norm);
    write_generated(&client_dir, &client_files)?;

    println!(
        "✅ Wrote {} transport, {} types, {} client stubs into {:?}",
        transport_files.len(),
        type_files.len(),
        client_files.len(),
        out_dir
    );
    Ok(())
}

/* --------------------------------------------------------------------- */
/*  Optional: discovery variant                                          */
/* --------------------------------------------------------------------- */

/// Same as [`run`] but discovers RPCs from a live `bitcoin-cli`.
pub fn run_discovery(bitcoind_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    tracing::info!(?bitcoind_path, ?out_dir, "Running discovery pipeline");
    // Discover --------------------------------------------------------------
    let discovered = rpc_method_discovery::discover_methods(bitcoind_path)
        .map_err(|e| anyhow::anyhow!("Discovery failed: {e}"))?;
    tracing::info!("Discovered {} methods", discovered.len());

    let helps: Vec<MethodHelp> = discovered
        .into_iter()
        .map(|m| MethodHelp {
            name: m.name,
            raw: m.description,
        })
        .collect();

    // Normalise & validate --------------------------------------------------
    let norm = DefaultSchemaNormalizer.normalize(&helps)?;
    DefaultSchemaValidator.validate(&norm)?;

    tracing::info!("Schema normalised + validated");

    // Delegate to the same writer paths ------------------------------------
    fs::create_dir_all(out_dir)?;
    let transport_dir = out_dir.join("transport");
    let client_dir = out_dir.join("client");
    let types_dir = out_dir.join("types");
    fs::create_dir_all(&transport_dir)?;
    fs::create_dir_all(&client_dir)?;
    fs::create_dir_all(&types_dir)?;

    write_generated(&transport_dir, &TransportCodeGenerator.generate(&norm))?;
    write_generated(&types_dir, &TypesCodeGenerator.generate(&norm))?;
    write_generated(&client_dir, &BasicCodeGenerator.generate(&norm))?;

    Ok(())
}
