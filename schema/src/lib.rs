// schema/src/lib.rs

pub mod normalize;
pub mod validator;

pub use normalize::{DefaultSchemaNormalizer, SchemaError as NormalizeError, SchemaNormalizer};
pub use validator::validate_numeric_value;
pub use validator::{DefaultSchemaValidator, SchemaValidator, ValidateError};
