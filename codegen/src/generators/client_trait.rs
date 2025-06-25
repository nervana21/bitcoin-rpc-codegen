// codegen/src/generators/client_trait.rs

use crate::utils::capitalize;
use crate::CodeGenerator;
use rpc_api::ApiMethod;
use type_registry::TypeRegistry;

/// Generator for creating Bitcoin RPC client traits for specific versions
pub struct ClientTraitGenerator {
    version: String,
}

impl ClientTraitGenerator {
    /// Create a new generator targeting a specific Bitcoin Core RPC version
    pub fn new(version: impl Into<String>) -> Self {
        ClientTraitGenerator {
            version: version.into(),
        }
    }
}

impl CodeGenerator for ClientTraitGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        // render client_trait.rs
        let template = include_str!("../../../templates/client_trait.rs");
        let client_trait = render_client_trait(template, methods, &self.version);

        // render mod.rs that re-exports the trait
        let version_no = self.version.replace('.', "");
        let mod_rs = format!(
            "//! Auto-generated module for BitcoinClient{version_no}\n\
             pub mod client_trait;\n\
             pub use self::client_trait::BitcoinClient{version_no};\n"
        );

        vec![
            ("client_trait.rs".into(), client_trait),
            ("mod.rs".into(), mod_rs),
        ]
    }
}

fn render_client_trait(template: &str, methods: &[ApiMethod], version: &str) -> String {
    let mut out = template.to_owned();

    // 1) version substitutions
    let version_no = version.replace('.', "");
    out = out.replace("{{VERSION}}", version);
    out = out.replace("{{VERSION_NODOTS}}", &version_no);

    // 2) imports (bring in all the generated `FooResponse` types)
    out = out.replace("{{IMPORTS}}", &build_imports());

    // 3) methods
    let trait_methods = methods
        .iter()
        .map(|m| MethodTemplate::new(m).render())
        .collect::<Vec<_>>()
        .join("\n\n");
    out.replace("{{TRAIT_METHODS}}", &trait_methods)
}

/// Bring in all the generated response types (e.g. `FooResponse`)
fn build_imports() -> String {
    ["crate::types::*", "std::future::Future"]
        .iter()
        .map(|p| format!("use {p};"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Tiny DSL to turn one ApiMethod into its doc-comment + fn
struct MethodTemplate<'a> {
    method: &'a ApiMethod,
}

impl<'a> MethodTemplate<'a> {
    fn new(method: &'a ApiMethod) -> Self {
        MethodTemplate { method }
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
        let args = self
            .method
            .arguments
            .iter()
            .map(|arg| {
                let name = if arg.names[0] == "type" {
                    "r#_type".to_string()
                } else {
                    format!("_{}", arg.names[0])
                };
                let (base_ty, _) = TypeRegistry.map_argument_type(arg);
                let ty = if arg.optional {
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

    /// Decide whether we return `()` or `FooResponse`
    fn return_type(&self) -> String {
        let none = self
            .method
            .results
            .first()
            .is_none_or(|r| r.type_.eq_ignore_ascii_case("none"));
        if none {
            "()".into()
        } else {
            format!("{}Response", capitalize(&self.method.name))
        }
    }

    /// Build the lines inside `vec![ ... ]`
    fn json_params(&self) -> String {
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

    fn render(&self) -> String {
        format!("{}\n{}", self.doc(), self.body())
    }
}
