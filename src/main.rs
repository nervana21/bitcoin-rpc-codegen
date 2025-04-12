use anyhow::Result;
use bitcoin_rpc_codegen::generator::main as generator_main;

fn main() -> Result<()> {
    generator_main()?;
    Ok(())
}
