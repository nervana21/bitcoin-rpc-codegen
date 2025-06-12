use crate::CodeGenerator;
use crate::TYPE_REGISTRY;
use rpc_api::ApiMethod;

/// Code generator that creates the client trait for Bitcoin RPC communication
pub struct ClientTraitGenerator {
    version: String,
}

impl ClientTraitGenerator {
    /// Creates a new `ClientTraitGenerator` configured for a specific RPC API version.
    ///
    /// The given `version` string will be used to suffix the generated trait and its implementation
    /// (e.g. `BitcoinClientV{version}`), ensuring the code you generate matches the intended
    /// Bitcoin Core RPC interface version.
    ///
    /// # Parameters
    ///
    /// - `version`: Any type convertible into a `String` that represents the target RPC API version
    ///   (for example, `"0.1.1"`).
    ///
    /// # Returns
    ///
    /// A new `ClientTraitGenerator` instance ready to emit client-trait code for the specified
    /// version of the Bitcoin RPC API.
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }
}

impl CodeGenerator for ClientTraitGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let template = include_str!("../../../templates/client_trait.rs");
        let rendered = render_client_trait(template, methods, &self.version);

        // also produce a mod.rs that re-exports the trait at the top level
        let version_no = self.version.replace('.', "");
        let mod_rs = format!(
            "//! Auto-generated module for BitcoinClientV{v}\n\
             pub mod client_trait;\n\
             pub use self::client_trait::BitcoinClientV{v};\n",
            v = version_no
        );
        vec![
            ("client_trait.rs".into(), rendered),
            ("mod.rs".into(), mod_rs),
        ]
    }
}

fn render_client_trait(template: &str, methods: &[ApiMethod], version: &str) -> String {
    let mut s = template.to_owned();

    // 1) Basic substitutions:
    let crate_ident = "bitcoin_rpc_midas"; // TODO: parameterize crate and version
    let version_no = version.replace('.', "");

    s = s.replace("{{CRATE_NAME}}", crate_ident);
    s = s.replace("{{VERSION}}", version);
    s = s.replace("{{VERSION_NODOTS}}", &version_no);

    // 2) Build the IMPORTS block:
    let imports = vec![
        "crate::transport::TransportExt".to_string(),
        "crate::transport::TransportError".to_string(),
        "crate::transport::core::wallet_methods::WALLET_METHODS".to_string(),
        "serde_json::Value".to_string(),
        "std::future::Future".to_string(),
        "serde::de::DeserializeOwned".to_string(),
    ]
    .into_iter()
    .map(|i| format!("use {};", i))
    .collect::<Vec<_>>()
    .join("\n");
    s = s.replace("{{IMPORTS}}", &imports);

    // 3) Build TRAIT_METHODS:
    let trait_methods = methods
        .iter()
        .map(|m| {
            let mut sig = String::new();
            // DOC comment
            for line in m.description.trim().lines() {
                sig.push_str(&format!("    /// {}\n", line.trim()));
            }

            // Build parameter list
            let params = m
                .arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "r#_type".to_string()
                    } else {
                        format!("_{}", arg.names[0].clone())
                    };
                    let (base_ty, _) = TYPE_REGISTRY.map_argument_type(arg);
                    let ty = if arg.optional {
                        format!("Option<{}>", base_ty)
                    } else {
                        base_ty.to_string()
                    };
                    format!("{}: {}", name, ty)
                })
                .collect::<Vec<_>>()
                .join(", ");

            // Add Rpc error stub
            sig.push_str(&format!(
                "    async fn {name}(&self{params}) -> Result<Value, TransportError> {{
        Err(TransportError::Rpc(\"Method {name} is not implemented\".to_string()))
    }}",
                name = m.name.to_lowercase(),
                params = if params.is_empty() {
                    "".into()
                } else {
                    format!(", {}", params)
                },
            ));
            sig
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    s = s.replace("{{TRAIT_METHODS}}", &trait_methods);

    // 4) Build IMPL_METHODS with wallet_call dispatch:
    let impl_methods = methods
        .iter()
        .map(|m| {
            // build signature
            let sig = {
                let list = m
                    .arguments
                    .iter()
                    .map(|arg| {
                        let name = if arg.names[0] == "type" {
                            "r#_type".to_string()
                        } else {
                            format!("_{}", arg.names[0].clone())
                        };
                        let (base_ty, _) = TYPE_REGISTRY.map_argument_type(arg);
                        let ty = if arg.optional {
                            format!("Option<{}>", base_ty)
                        } else {
                            base_ty.to_string()
                        };
                        format!("{}: {}", name, ty)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                if list.is_empty() {
                    "".into()
                } else {
                    format!(", {}", list)
                }
            };

            // build JSON param list
            let args = m
                .arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "r#_type".to_string()
                    } else {
                        format!("_{}", arg.names[0].clone())
                    };
                    format!("        serde_json::json!({}),", name)
                })
                .collect::<Vec<_>>()
                .join("\n");

            // dispatch on WALLET_METHODS
            let rpc_name = &m.name;
            let call_expr = format!(
                "self.dispatch_json::<Value>(\"{rpc}\", &params).await",
                rpc = rpc_name
            );

            // assemble the method body
            format!(
                "    async fn {name}(&self{sig}) -> Result<Value, TransportError> {{
        // build params
        let params = vec![
{args}
        ];
        // dispatch to node or wallet namespace
        Ok({call_expr}?.into())
    }}",
                name = m.name.to_lowercase(),
                sig = sig,
                args = args,
                call_expr = call_expr
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    s = s.replace("{{IMPL_METHODS}}", &impl_methods);

    s
}
