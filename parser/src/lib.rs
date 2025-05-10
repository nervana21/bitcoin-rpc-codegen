// parser/src/lib.rs
//! The `parser` crate: tokenizes `bitcoin-cli help` output into method blocks.
use thiserror::Error;

pub mod discover;
pub use discover::{discover_methods, parse_help_output, DiscoverError};

/// A raw help block for one RPC method.
#[derive(Debug, Clone)]
pub struct MethodHelp {
    /// The RPC method's name, e.g. "getblockchaininfo"
    pub name: String,
    /// The full help text (all lines) for that method
    pub raw: String,
}

/// Errors that can occur during help‑text parsing.
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("no methods found")]
    NoMethods,
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// A help‑text parser: input is the entire `bitcoin-cli help` output,
/// output is a Vec of `MethodHelp` blocks.
pub trait HelpParser {
    fn parse(&self, raw_help: &str) -> Result<Vec<MethodHelp>, ParserError>;
}

/// The "standard" help parser: groups each signature plus following description
/// under a single MethodHelp, ignoring blank lines and category headings.
pub struct DefaultHelpParser;

impl HelpParser for DefaultHelpParser {
    fn parse(&self, raw_help: &str) -> Result<Vec<MethodHelp>, ParserError> {
        let mut methods = Vec::new();
        let mut current_name: Option<String> = None;
        let mut current_raw: Vec<String> = Vec::new();

        for line in raw_help.lines() {
            let trimmed_end = line.trim_end();
            let trimmed = trimmed_end.trim_start();

            // Skip blank lines and category headings
            if trimmed.is_empty() || trimmed.starts_with("=") {
                continue;
            }

            let first_char = trimmed.chars().next().unwrap();
            if first_char.is_ascii_lowercase() {
                // New method signature line
                if let Some(name) = current_name.take() {
                    methods.push(MethodHelp {
                        name,
                        raw: current_raw.join("\n"),
                    });
                }
                // Start a new block
                let name_tok = trimmed.split_whitespace().next().unwrap().to_string();
                current_name = Some(name_tok.clone());
                current_raw.clear();
                current_raw.push(trimmed.to_string());
            } else {
                // Description or continuation line
                if current_name.is_some() {
                    current_raw.push(trimmed.to_string());
                }
            }
        }
        // Push last buffered method
        if let Some(name) = current_name.take() {
            methods.push(MethodHelp {
                name,
                raw: current_raw.join("\n"),
            });
        }

        if methods.is_empty() {
            Err(ParserError::NoMethods)
        } else {
            Ok(methods)
        }
    }
}
