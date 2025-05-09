use std::{fs, path::PathBuf};

use anyhow::Result;
use clap::Parser as ClapParser;
use codegen::{write_generated, CodeGenerator, TransportCodeGenerator};
use parser::{DefaultHelpParser, HelpParser};
use schema::{DefaultSchemaNormalizer, DefaultSchemaValidator, SchemaNormalizer, SchemaValidator};

/// Simple pipeline: parse help → normalize → generate RPC modules.
#[derive(ClapParser)]
#[command(author, version, about)]
struct Opts {
    /// Path to the `bitcoin-cli help` dump
    #[arg(short, long, default_value = "help.txt")]
    input: PathBuf,

    /// Where to write generated modules (e.g. client/src/generated)
    #[arg(short, long, default_value = "../client/src/generated")]
    out_dir: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    // 1) Read help dump
    let help = fs::read_to_string(&opts.input)?;

    // 2) Parse into ApiMethod structs
    let raw = DefaultHelpParser.parse(&help)?;

    // 3) Normalize & validate
    let norm = DefaultSchemaNormalizer.normalize(&raw)?;
    DefaultSchemaValidator.validate(&norm)?;

    // 4) Generate modules
    let gen = TransportCodeGenerator;
    let files = gen.generate(&norm);

    // 5) Write into client crate
    write_generated(&opts.out_dir, &files)?;

    println!(
        "✅ Wrote {} RPC modules into {:?}",
        files.len(),
        opts.out_dir
    );
    Ok(())
}
