//! Shared Bitcoin RPC type definitions
//!
//! This crate provides the core types and utilities for working with the
//! Bitcoin Core JSON-RPC API. It serves as a shared foundation that can be
//! used by both the codegen system and external consumers like fuzzing tools.
//!
//! ## Core Types
//!
//! - `BtcMethod` - Complete Bitcoin method definition
//! - `BtcArgument` - Method argument specification
//! - `BtcResult` - Method result specification
//! - `ApiDefinition` - Complete API definition container
//!
//! ## Features
//!
//! - JSON serialization/deserialization with serde
//! - Schema loading utilities
//! - Type conversion helpers
//! - Error handling with thiserror

pub mod types;

pub use types::{
    ApiDefinition, BtcArgument, BtcMethod, BtcResult, Result, SchemaError, Version,
    VersionError,
};
