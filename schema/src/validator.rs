// schema/src/validator.rs

use thiserror::Error;

use rpc_api::ApiMethod;

/// Errors from validation
#[derive(Debug, Error)]
pub enum ValidateError {
    #[error("method with empty name found")]
    EmptyName,
    #[error("duplicate method name: {0}")]
    DuplicateName(String),
}

/// Trait for sanity‑checking a schema
pub trait SchemaValidator {
    fn validate(&self, schema: &[ApiMethod]) -> Result<(), ValidateError>;
}

/// Default: ensure every method has a name and no duplicates
pub struct DefaultSchemaValidator;

impl SchemaValidator for DefaultSchemaValidator {
    fn validate(&self, schema: &[ApiMethod]) -> Result<(), ValidateError> {
        let mut seen = std::collections::HashSet::new();
        for m in schema {
            if m.name.is_empty() {
                return Err(ValidateError::EmptyName);
            }
            if !seen.insert(&m.name) {
                return Err(ValidateError::DuplicateName(m.name.clone()));
            }
        }
        Ok(())
    }
}
