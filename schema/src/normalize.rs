// schema/src/normalize.rs

use thiserror::Error;

use parser::MethodHelp;
use rpc_api::ApiMethod;

/// Errors from normalization
#[derive(Debug, Error)]
pub enum SchemaError {
    #[error("no help blocks provided")]
    NoHelpBlocks,
    // TODO: add more detailed parse errors here
}

/// Trait to convert raw help into a structured schema
pub trait SchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError>;
}

/// Default: one‑to‑one mapping; description = raw text; empty args/results
pub struct DefaultSchemaNormalizer;

impl SchemaNormalizer for DefaultSchemaNormalizer {
    fn normalize(&self, helps: &[MethodHelp]) -> Result<Vec<ApiMethod>, SchemaError> {
        if helps.is_empty() {
            return Err(SchemaError::NoHelpBlocks);
        }
        Ok(helps
            .iter()
            .map(|mh| ApiMethod {
                name: mh.name.clone(),
                description: mh.raw.clone(),
                arguments: Vec::new(),
                results: Vec::new(),
            })
            .collect())
    }
}
