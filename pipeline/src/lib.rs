// pipeline/src/lib.rs

use anyhow::Result;
use codegen::{write_generated, CodeGenerator, TransportCodeGenerator};
use parser::{DefaultHelpParser, HelpParser};
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};
use std::fs;
use std::path::PathBuf;

/// Run the RPC code generation pipeline
///
/// # Arguments
///
/// * `input_path` - Path to the `bitcoin-cli help` dump
/// * `out_dir` - Where to write generated modules (e.g. client/src/generated)
pub fn run(input_path: &PathBuf, out_dir: &PathBuf) -> Result<()> {
    // 1) Read help dump
    let help = fs::read_to_string(input_path)?;

    // 2) Parse into ApiMethod structs
    let raw = DefaultHelpParser.parse(&help)?;

    // 3) Normalize & validate
    let norm = DefaultSchemaNormalizer.normalize(&raw)?;
    DefaultSchemaValidator.validate(&norm)?;

    // 4) Generate modules
    let gen = TransportCodeGenerator;
    let files = gen.generate(&norm);

    // 5) Write into client crate
    write_generated(out_dir, &files)?;

    println!("✅ Wrote {} RPC modules into {:?}", files.len(), out_dir);
    Ok(())
}
