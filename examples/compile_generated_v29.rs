// examples/compile_generated_v29.rs

//! This test exists to confirm that the output from `generate_v29.rs`
//! results in valid, compiling Rust code. It is the final trust step
//! between structured schema and working client types.

fn main() {
    println!("🚧 This test must be run manually with rustc or cargo check.");
    println!("Try:");
    println!("   cargo check --manifest-path examples/compile_generated_v29/Cargo.toml");
}
