// codegen/src/generators/client_trait.rs

use bitcoin_rpc_conversions::TypeRegistry;
use bitcoin_rpc_types::BtcMethod;

use crate::utils::capitalize;
use crate::CodeGenerator;

/// Generator for creating Bitcoin RPC client traits for specific versions
pub struct ClientTraitGenerator {
    version: String,
}

impl ClientTraitGenerator {
    /// Create a new generator targeting a specific Bitcoin Core RPC version
    pub fn new(version: impl Into<String>) -> Self {
        ClientTraitGenerator { version: version.into() }
    }
}

impl CodeGenerator for ClientTraitGenerator {
    fn generate(&self, methods: &[BtcMethod]) -> Vec<(String, String)> {
        // render client_trait.rs
        let template = include_str!("../../../templates/client_trait.rs");
        let client_trait = render_client_trait(template, methods, &self.version);

        // render mod.rs that re-exports the trait
        let version_no = format!(
            "V{}",
            self.version.trim_start_matches('v').trim_start_matches('V').replace('.', "_")
        );
        let mod_rs = format!(
            "//! Auto-generated module for BitcoinClient{version_no}\n\
             pub mod client;\n\
             pub use self::client::BitcoinClient{version_no};\n"
        );

        vec![("client.rs".into(), client_trait), ("mod.rs".into(), mod_rs)]
    }
}

/// Render the client trait
pub fn render_client_trait(template: &str, methods: &[BtcMethod], version: &str) -> String {
    let mut out = template.to_owned();

    let version_no =
        format!("V{}", version.trim_start_matches('v').trim_start_matches('V').replace('.', "_"));
    out = out.replace("{{VERSION}}", version);
    out = out.replace("{{VERSION_NODOTS}}", &version_no);

    out = out.replace("{{IMPORTS}}", &build_imports());

    let param_structs = methods
        .iter()
        .filter_map(|m| MethodTemplate::new(m).generate_param_struct())
        .collect::<Vec<_>>()
        .join("\n\n");
    out = out.replace("{{PARAM_STRUCTS}}", &param_structs);

    let trait_methods =
        methods.iter().map(|m| MethodTemplate::new(m).render()).collect::<Vec<_>>().join("\n\n");
    out.replace("{{TRAIT_METHODS}}", &trait_methods)
}

/// Bring in all the generated response types (e.g. `FooResponse`)
fn build_imports() -> String {
    ["crate::responses::*", "std::future::Future", "bitcoin_rpc_types::HashOrHeight"]
        .iter()
        .map(|p| format!("use {p};"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Tiny DSL to turn one BtcMethod into its doc-comment + fn
pub struct MethodTemplate<'a> {
    method: &'a BtcMethod,
}

impl<'a> MethodTemplate<'a> {
    /// Create a new MethodTemplate for the given BtcMethod
    pub fn new(method: &'a BtcMethod) -> Self { MethodTemplate { method } }

    /// Generate parameter struct for methods that require argument reordering
    pub fn generate_param_struct(&self) -> Option<String> {
        use crate::utils::{needs_parameter_reordering, reorder_arguments_for_rust_signature};

        if !needs_parameter_reordering(&self.method.arguments) {
            return None;
        }

        let (reordered_args, param_mapping) =
            reorder_arguments_for_rust_signature(&self.method.arguments);
        let struct_name = format!("{}Params", capitalize(&self.method.name));

        let mut fields = Vec::new();
        for arg in &reordered_args {
            let field_name = if arg.names[0] == "type" {
                "r#_type".to_string()
            } else {
                format!("_{}", arg.names[0])
            };

            let (base_ty, _) = TypeRegistry::map_argument_type(arg);
            let field_type =
                if !arg.required { format!("Option<{base_ty}>") } else { base_ty.to_string() };

            fields.push(format!("    pub {}: {},", field_name, field_type));
        }

        // Generate custom serialization that converts struct to array in original order
        let mut serialize_fields = Vec::new();
        for (original_idx, _) in self.method.arguments.iter().enumerate() {
            let reordered_idx = param_mapping.iter().position(|&x| x == original_idx).unwrap();
            let arg = &reordered_args[reordered_idx];
            let field_name =
                if arg.names[0] == "type" { "r#_type" } else { &format!("_{}", arg.names[0]) };
            serialize_fields.push(format!("        seq.serialize_element(&self.{})?;", field_name));
        }

        Some(format!(
            "#[derive(Debug, Clone, Deserialize)]\n\
            pub struct {} {{\n\
            {}\n\
            }}\n\
            \n\
            impl serde::Serialize for {} {{\n\
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>\n\
                where\n\
                    S: serde::Serializer,\n\
                {{\n\
                    let mut seq = serializer.serialize_seq(Some({}))?;\n\
            {}\n\
                    seq.end()\n\
                }}\n\
            }}",
            struct_name,
            fields.join("\n"),
            struct_name,
            self.method.arguments.len(),
            serialize_fields.join("\n")
        ))
    }

    /// Render the /// doc lines
    fn doc(&self) -> String {
        self.method
            .description
            .trim()
            .lines()
            .map(|l| format!("    /// {}", l.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Build the `, name: Type, ...` part of the fn signature
    fn signature(&self) -> String {
        use crate::utils::needs_parameter_reordering;

        // Check if this method requires argument reordering
        if needs_parameter_reordering(&self.method.arguments) {
            // Use a parameter struct for methods with ordering issues
            let struct_name = format!("{}Params", capitalize(&self.method.name));
            format!(", params: {}", struct_name)
        } else {
            // Use individual parameters for methods that don't require argument reordering
            let args = self
                .method
                .arguments
                .iter()
                .map(|arg| {
                    // Add underscore prefix to all parameter names for consistency and clarity.
                    // This distinguishes parameters from other identifiers and follows Rust conventions
                    // for intentionally prefixed names. The special case for "type" uses r#_type
                    // to properly escape the reserved keyword.
                    let name = if arg.names[0] == "type" {
                        "r#_type".to_string()
                    } else {
                        format!("_{}", arg.names[0])
                    };
                    let (base_ty, _) = TypeRegistry::map_argument_type(arg);
                    let ty = if !arg.required {
                        format!("Option<{base_ty}>")
                    } else {
                        base_ty.to_string()
                    };
                    format!("{name}: {ty}")
                })
                .collect::<Vec<_>>();
            if args.is_empty() {
                "".into()
            } else {
                format!(", {}", args.join(", "))
            }
        }
    }

    /// Decide whether we return `()` or `FooResponse`
    fn return_type(&self) -> String {
        let none = self.method.results.first().is_none_or(|r| r.type_.eq_ignore_ascii_case("none"));
        if none {
            "()".into()
        } else {
            format!("{}Response", capitalize(&self.method.name))
        }
    }

    /// Build the lines inside `vec![ ... ]`
    pub fn json_params(&self) -> String {
        use crate::utils::needs_parameter_reordering;

        if needs_parameter_reordering(&self.method.arguments) {
            // For methods that require argument reordering, serialize from the parameter struct
            "            serde_json::json!(params),".to_string()
        } else {
            // For methods not needing reordering, serialize individual parameters
            self.method
                .arguments
                .iter()
                .map(|arg| {
                    let name = if arg.names[0] == "type" {
                        "r#_type"
                    } else {
                        &format!("_{}", arg.names[0])
                    };
                    format!("            serde_json::json!({name}),")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
    }

    /// Assemble the full async fn stub
    fn body(&self) -> String {
        let name = self.method.name.to_lowercase();
        let sig = self.signature();
        let ret = self.return_type();
        let json = self.json_params();
        let rpc = &self.method.name;

        format!(
            "    async fn {name}(&self{sig}) -> Result<{ret}, TransportError> {{
        let params = vec![
{json}
        ];
        self.dispatch_json::<{ret}>(\"{rpc}\", &params).await
    }}"
        )
    }

    fn render(&self) -> String { format!("{}\n{}", self.doc(), self.body()) }
}
