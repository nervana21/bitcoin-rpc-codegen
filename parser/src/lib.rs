// parser/src/lib.rs
//! The `parser` crate: tokenizes `bitcoin-cli help` output into method blocks.
use thiserror::Error;

/// A raw help block for one RPC method.
#[derive(Debug, Clone)]
pub struct MethodHelp {
    /// The RPC method’s name, e.g. "getblockchaininfo"
    pub name: String,
    /// The full help text (all lines) for that method
    pub raw: String,
}

/// Errors that can occur during help‐text parsing.
#[derive(Error, Debug)]
pub enum ParserError {
    #[error("no methods found")]
    NoMethods,
    #[error("regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // TODO: add more cases as needed
}

/// A help‐text parser: input is the entire `bitcoin-cli help` output,
/// output is a Vec of (name, raw‐text) blocks.
pub trait HelpParser {
    fn parse(&self, raw_help: &str) -> Result<Vec<MethodHelp>, ParserError>;
}

/// The “standard” help parser: splits on blank lines and captures the first token as name.
pub struct DefaultHelpParser;

impl HelpParser for DefaultHelpParser {
    fn parse(&self, raw_help: &str) -> Result<Vec<MethodHelp>, ParserError> {
        // Simple first cut: split on double‐newlines
        let mut methods = Vec::new();
        for block in raw_help.trim().split("\n\n") {
            let block = block.trim();
            if block.is_empty() {
                continue;
            }
            // First word of the first line is the method name
            let first_line = block.lines().next().unwrap();
            let name = first_line
                .split_whitespace()
                .next()
                .ok_or(ParserError::NoMethods)?
                .to_string();
            methods.push(MethodHelp {
                name,
                raw: block.to_string(),
            });
        }
        if methods.is_empty() {
            Err(ParserError::NoMethods)
        } else {
            Ok(methods)
        }
    }
}
