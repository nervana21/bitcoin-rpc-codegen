use std::env;
use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Get input path from first argument, or use default
    let input_path = args
        .get(1)
        .filter(|arg| arg.as_str() != "pipeline") // Ignore the "pipeline" argument from cargo run
        .map(PathBuf::from)
        .or_else(|| {
            let default = PathBuf::from("bitcoin-core-api.json");
            if default.exists() {
                Some(default)
            } else {
                None
            }
        });

    pipeline::run(input_path.as_ref())
}
