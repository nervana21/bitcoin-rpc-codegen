//! Generate subclients for node and wallet operations

use crate::generators::doc_comment;
use crate::utils::{camel_to_snake_case, rust_type_for_argument};
use rpc_api::ApiMethod;
use std::fmt::Write;

use super::utils::camel;

/// Generates a complete Rust client struct and implementation for a collection of Bitcoin RPC methods.
///
/// This function creates a test-only client that wraps a transport layer and provides
/// async methods for each RPC call. The generated client:
/// - Converts RPC method names to snake_case for Rust conventions
/// - Handles parameter serialization to JSON values
/// - Provides proper error handling with TransportError
/// - Includes documentation comments from the API specification
/// - Supports both void and typed return values
///
/// The generated code is wrapped in `#[cfg(test)]` to ensure it's only available
/// during testing, making it suitable for integration tests with Bitcoin nodes.
pub fn generate_subclient(
    client_name: &str,
    methods: &[ApiMethod],
    version: &str,
) -> std::io::Result<String> {
    let mut code = String::new();

    writeln!(code, "#[cfg(test)]").unwrap();

    let needs_value = methods.iter().any(|m| {
        !m.arguments.is_empty()
            || (m.results.len() == 1 && m.results[0].type_.to_lowercase() != "none")
    });

    let version_lowercase = version.to_lowercase();

    writeln!(
        code,
        "use anyhow::Result;
use std::sync::Arc;
use crate::transport::core::{{TransportExt, TransportError}};
use crate::transport::{{DefaultTransport}};
use crate::types::{}_types::*;
{}",
        version_lowercase,
        if needs_value {
            "#[cfg(test)]\nuse serde_json::Value;\n"
        } else {
            ""
        }
    )
    .map_err(std::io::Error::other)?;

    writeln!(
        code,
        "#[derive(Debug, Clone)]
pub struct {client_name} {{
    client: Arc<DefaultTransport>,
}}

impl {client_name} {{
    pub fn new(client: Arc<DefaultTransport>) -> Self {{
        Self {{ client }}
    }}

    pub fn with_transport(&mut self, client: Arc<DefaultTransport>) {{
        self.client = client;
    }}"
    )
    .map_err(std::io::Error::other)?;

    for m in methods {
        let method_snake = camel_to_snake_case(&m.name);
        let returns_unit = m.results.is_empty() || m.results[0].type_.to_lowercase() == "none";
        let ret_ty = if returns_unit {
            "()".to_string()
        } else {
            format!("{}Response", camel(&m.name))
        };

        writeln!(
            code,
            "\n{}",
            doc_comment::format_doc_comment(&m.description)
        )
        .map_err(std::io::Error::other)?;

        let params_sig = if m.arguments.is_empty() {
            "".to_string()
        } else {
            m.arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "_type".to_string()
                    } else {
                        camel_to_snake_case(&arg.names[0])
                    };
                    let ty = rust_type_for_argument(&arg.names[0], &arg.type_);
                    format!("{name}: {ty}")
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        writeln!(
            code,
            "    pub async fn {method}(&self{sig}) -> Result<{ret}, TransportError> {{",
            method = method_snake,
            sig = if params_sig.is_empty() {
                "".into()
            } else {
                format!(", {params_sig}")
            },
            ret = ret_ty,
        )
        .map_err(std::io::Error::other)?;

        if !m.arguments.is_empty() {
            writeln!(code, "        let mut params = Vec::new();")
                .map_err(std::io::Error::other)?;
            for arg in &m.arguments {
                let name = if arg.names[0] == "type" {
                    "_type"
                } else {
                    &camel_to_snake_case(&arg.names[0])
                };
                writeln!(code, "        params.push(serde_json::to_value({name})?);")
                    .map_err(std::io::Error::other)?;
            }
        }

        writeln!(code, "        // dispatch and deserialize to `{ret_ty}`")
            .map_err(std::io::Error::other)?;
        writeln!(
            code,
            "        self.client.call::<{ret}>(\"{rpc}\", &{vec}).await",
            ret = ret_ty,
            rpc = m.name,
            vec = if m.arguments.is_empty() {
                "[]".to_string()
            } else {
                "params".to_string()
            },
        )
        .map_err(std::io::Error::other)?;

        writeln!(code, "    }}").map_err(std::io::Error::other)?;
    }

    writeln!(code, "}}\n").map_err(std::io::Error::other)?;
    Ok(code)
}
