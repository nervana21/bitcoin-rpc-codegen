// use std::{env, fs, path::PathBuf};

// Make these modules public so they are visible as crate::parser and crate::generator
#[path = "src/parser/mod.rs"]
pub mod parser;

#[path = "src/generator/mod.rs"]
pub mod generator;

#[path = "src/generator/codegen.rs"]
pub mod codegen;

fn main() {
    // Instruct Cargo to re-run this build script when the API JSON changes.
    println!("cargo:rerun-if-changed=resources/api.json");

    // Call the shared code generation function.
    codegen::run_codegen().expect("Code generation failed in build.rs");
}
