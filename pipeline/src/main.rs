use std::env;
use std::path::PathBuf;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).map(PathBuf::from);

    pipeline::run(input_path.as_ref())
}
