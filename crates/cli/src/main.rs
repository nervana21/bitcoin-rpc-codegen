use anyhow::Result;
use clap::{Parser, Subcommand};
use flate2::read::GzDecoder;
use std::io::BufReader;
use std::{fs::File, path::PathBuf};
use tar::Archive;

/// CLI for bitcoin-rpc-codegen extract workflow
#[derive(Parser)]
#[command(name = "bitcoin-rpc-codegen-cli")]
#[command(about = "CLI for bitcoin-rpc-codegen extract workflow", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract a .tar.gz archive
    Extract {
        /// Path to the .tar.gz archive
        #[arg(long)]
        archive: String,
        /// Destination directory for extraction
        #[arg(long)]
        output: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Extract { archive, output } => {
            let archive_path = PathBuf::from(&archive);
            let output_dir = PathBuf::from(&output);
            println!(
                "Extracting {} → {}",
                archive_path.display(),
                output_dir.display()
            );

            // Open archive file
            let file = File::open(&archive_path)?;
            let buf = BufReader::new(file);

            // Decompress and unpack
            let decoder = GzDecoder::new(buf);
            let mut archive = Archive::new(decoder);
            archive.unpack(&output_dir)?;

            println!("Extraction complete.");
        }
    }

    Ok(())
}
