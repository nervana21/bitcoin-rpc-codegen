// schema/src/validator.rs

use rpc_api::ApiMethod;
use serde_json::Value;
use thiserror::Error;

/// Errors from validation
#[derive(Debug, Error)]
pub enum ValidateError {
    #[error("method with empty name found")]
    EmptyName,
    #[error("duplicate method name: {0}")]
    DuplicateName(String),
}

/// Errors from numeric validation
#[derive(Debug, Error)]
pub enum NumericValidationError {
    #[error("expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },
    #[error("invalid amount: {0}")]
    InvalidAmount(f64),
    #[error("amount out of range: {0}")]
    AmountOutOfRange(f64),
}

/// Numeric type enum for validation
#[derive(Debug, Clone, Copy)]
pub enum NumericType {
    U64,
    F64,
    BitcoinAmount,
    Other,
}

impl From<&str> for NumericType {
    fn from(s: &str) -> Self {
        match s {
            "u64" => NumericType::U64,
            "f64" => NumericType::F64,
            "bitcoin::Amount" => NumericType::BitcoinAmount,
            _ => NumericType::Other,
        }
    }
}

impl NumericType {
    /// Validate a JSON value against this numeric type
    pub fn validate(&self, value: &Value) -> Result<(), NumericValidationError> {
        match self {
            NumericType::U64 => {
                value
                    .as_u64()
                    .map(|_| ())
                    .ok_or_else(|| NumericValidationError::TypeMismatch {
                        expected: "u64".into(),
                        actual: value.to_string(),
                    })
            }
            NumericType::F64 => {
                value
                    .as_f64()
                    .map(|_| ())
                    .ok_or_else(|| NumericValidationError::TypeMismatch {
                        expected: "f64".into(),
                        actual: value.to_string(),
                    })
            }
            NumericType::BitcoinAmount => {
                let amount =
                    value
                        .as_f64()
                        .ok_or_else(|| NumericValidationError::TypeMismatch {
                            expected: "bitcoin::Amount".into(),
                            actual: value.to_string(),
                        })?;

                if amount < 0.0 {
                    return Err(NumericValidationError::InvalidAmount(amount));
                }
                if amount > 21_000_000.0 {
                    return Err(NumericValidationError::AmountOutOfRange(amount));
                }

                // Enforce max 8 decimal places by checking satoshi integer
                let satoshis = (amount * 1e8).round();
                let reconstructed = satoshis / 1e8;
                if (reconstructed - amount).abs() > f64::EPSILON {
                    return Err(NumericValidationError::InvalidAmount(amount));
                }

                Ok(())
            }
            NumericType::Other => {
                // Treat unknown numeric types as an explicit mismatch
                Err(NumericValidationError::TypeMismatch {
                    expected: "numeric type".into(),
                    actual: value.to_string(),
                })
            }
        }
    }
}

/// Validates numeric values according to Bitcoin Core's type system.
///
/// # Supported Types
/// - `u64`: For block heights, transaction counts, etc.
/// - `f64`: For mining difficulty, fee rates, etc.
/// - `bitcoin::Amount`: For transaction amounts, fees, etc.
///   - Must be between 0 and 21,000,000 BTC
///   - Must have at most 8 decimal places (satoshi precision)
/// - Other numeric types will fail validation
pub fn validate_numeric_value(
    value: &Value,
    expected_type: &str,
) -> Result<(), NumericValidationError> {
    NumericType::from(expected_type).validate(value)
}

/// Trait for sanity-checking a schema
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_difficulty_validation() {
        // Valid floating-point difficulty
        let v = json!(1234.5678);
        assert!(validate_numeric_value(&v, "f64").is_ok());
        // Integer is also fine
        let v = json!(1234);
        assert!(validate_numeric_value(&v, "f64").is_ok());
        // Non-numeric fails
        let v = json!("oops");
        assert!(validate_numeric_value(&v, "f64").is_err());
    }

    #[test]
    fn test_amount_validation() {
        // Valid amount
        let v = json!(0.0001);
        assert!(validate_numeric_value(&v, "bitcoin::Amount").is_ok());
        // Too large
        let v = json!(22_000_000.0);
        assert!(validate_numeric_value(&v, "bitcoin::Amount").is_err());
        // Negative
        let v = json!(-1.0);
        assert!(validate_numeric_value(&v, "bitcoin::Amount").is_err());
        // Too many decimals
        let v = json!(0.000000001);
        assert!(validate_numeric_value(&v, "bitcoin::Amount").is_err());
    }

    #[test]
    fn test_unknown_type() {
        let v = json!(42);
        assert!(validate_numeric_value(&v, "mystery").is_err());
    }
}
