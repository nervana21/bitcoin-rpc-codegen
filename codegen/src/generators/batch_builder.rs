// codegen/src/generators/batch_builder.rs

use crate::{utils::capitalize, CodeGenerator};
use rpc_api::ApiMethod;
use type_registry::TYPE_REGISTRY;

use std::fmt::Write;

/// Generates a fluent `BatchBuilder` with one method-per-RPC and an `.execute()` entrypoint.
pub struct BatchBuilderGenerator;

impl CodeGenerator for BatchBuilderGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut code = String::new();

        // Imports
        writeln!(
            code,
            "use std::sync::Arc;
use crate::transport::{{TransportTrait, TransportError, BatchTransport}};
use serde_json::{{Value, json}};
use serde::Deserialize;
use crate::types::*;"
        )
        .unwrap();

        // Generate BatchResults struct
        writeln!(
            code,
            r#"/// Typed results for a JSON-RPC batch
#[derive(Debug, Deserialize)]
pub struct BatchResults {{
"#
        )
        .unwrap();

        // Add fields for each method
        for m in methods {
            let field_name = m.name.clone();

            // Check if this method returns void (no results or all results are "none")
            let returns_unit = m.results.is_empty() || m.results.iter().all(|r| r.type_ == "none");

            if returns_unit {
                // For void methods, use () as the response type
                writeln!(code, "    pub {field_name}: (),").unwrap();
            } else {
                // For non-void methods, always use Option<T> since we may not call every method in a batch
                // TODO: Remove the superfluous Option wrapper by making only batched methods Option<T>,
                // or switch to a per-batch struct that only includes the fields that are actually queued (more involved).
                let response_type = if m.results.len() == 1 {
                    let (ty, _) = TYPE_REGISTRY.map_result_type(&m.results[0]);
                    if ty == "()" {
                        "()".to_string()
                    } else {
                        format!("Option<{}Response>", capitalize(&m.name))
                    }
                } else {
                    format!("Option<{}Response>", capitalize(&m.name))
                };
                writeln!(code, "    pub {field_name}: {response_type},").unwrap();
            }
        }

        writeln!(code, "}}\n").unwrap();

        // Builder struct
        writeln!(
            code,
            r#"/// Fluent builder for batching multiple RPC calls
pub struct BatchBuilder {{
    tx: BatchTransport,
    calls: Vec<(&'static str, Vec<Value>)>,
}}
"#
        )
        .unwrap();

        // Impl block
        writeln!(
            code,
            r#"impl BatchBuilder {{
    /// Wraps a transport and begins a batch
    pub fn new(inner: Arc<dyn TransportTrait>) -> Self {{
        let tx = BatchTransport::new(inner);
        tx.begin_batch();
        BatchBuilder {{ tx, calls: Vec::new() }}
    }}

"#
        )
        .unwrap();

        // Generate one queueing method per RPC
        for m in methods {
            let name = m.name.clone();
            // Build argument list
            let args_list = m
                .arguments
                .iter()
                .map(|arg| {
                    let arg_name = if arg.names[0] == "type" {
                        format!("r#{}", arg.names[0])
                    } else {
                        arg.names[0].clone()
                    };
                    format!(", {arg_name}: Value")
                })
                .collect::<Vec<_>>()
                .join("");
            // Build params vec literal
            let params = if m.arguments.is_empty() {
                "Vec::new()".to_string()
            } else {
                let elems = m
                    .arguments
                    .iter()
                    .map(|arg| {
                        let arg_name = if arg.names[0] == "type" {
                            format!("r#{}", arg.names[0])
                        } else {
                            arg.names[0].clone()
                        };
                        format!("json!({arg_name})")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("vec![{elems}]")
            };

            writeln!(
                code,
                r#"    /// Queue a `{name}` RPC call
    pub fn {name}(mut self{args_list}) -> Self {{
        self.calls.push(("{name}", {params}));
        self
    }}
"#
            )
            .unwrap();
        }

        // Execute with typed results
        writeln!(
            code,
            r#"    /// Executes the batch and returns typed results
    pub async fn execute(self) -> Result<BatchResults, TransportError> {{
        let BatchBuilder {{tx, calls }} = self;
        // queue all calls into the transport
        for (method, params) in &calls {{
            let _ = tx.send_request(method, params);
        }}
        let raw_results = tx.end_batch()
            .await
            .map_err(|e| TransportError::Rpc(e.to_string()))?;
        
        // Parse the raw results into our typed struct
        let mut results = BatchResults {{
"#
        )
        .unwrap();

        // Generate field assignments - initialize all to None/()
        for m in methods {
            let field_name = m.name.clone();
            let returns_unit = m.results.is_empty() || m.results.iter().all(|r| r.type_ == "none");

            if returns_unit {
                writeln!(code, "            {field_name}: (),").unwrap();
            } else {
                // Always initialize to None since we may not call every method
                writeln!(code, "            {field_name}: None,").unwrap();
            }
        }

        writeln!(
            code,
            r#"        }};
        
        // Populate the fields based on the actual calls made
        for (i, (method_name, _)) in calls.iter().enumerate() {{
            match *method_name {{
"#
        )
        .unwrap();

        // Generate match arms for each method
        for method in methods {
            let field_name = method.name.clone();
            let returns_unit =
                method.results.is_empty() || method.results.iter().all(|r| r.type_ == "none");

            if returns_unit {
                writeln!(
                    code,
                    r#"                "{}" => results.{} = (),"#,
                    method.name, field_name
                )
                .unwrap();
            } else {
                // Always wrap in Some() since all non-void fields are Option<T> in batch context
                writeln!(
                    code,
                    r#"                "{}" => results.{} = Some(serde_json::from_value::<{}Response>(raw_results[i].clone())?),"#,
                    method.name, field_name, capitalize(&method.name)
                )
                .unwrap();
            }
        }

        writeln!(
            code,
            r#"                _ => return Err(TransportError::Rpc(format!("Unknown method: {{}}", method_name))),
            }}
        }}
        
        Ok(results)
    }}
}}
"#
        )
        .unwrap();

        vec![("batch_builder.rs".to_string(), code)]
    }
}
