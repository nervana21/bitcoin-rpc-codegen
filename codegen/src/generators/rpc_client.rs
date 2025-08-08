// codegen/src/rpc_client_generator.rs
//
// ──────────────────────────────────────────────────────────────────────────────
//  RpcClientGenerator
//  ----------------------
//  Generates *callable* client methods for every `ApiMethod`:
//
//  • async fn with typed parameters (required first, optional last)
//  • builds a `Vec<serde_json::Value>` and invokes `self.call_method`
//  • if the RPC returns non‑null → inlines a `CamelCaseMethodNameResponse`
//    struct and deserialises into it; else returns `()`
// ──────────────────────────────────────────────────────────────────────────────
use crate::generators::doc_comment;
use crate::utils::camel_to_snake_case;
use crate::CodeGenerator;
use rpc_api::{ApiArgument, ApiMethod, ApiResult};
use std::fmt::Write as _;
use type_registry::TypeRegistry;

fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut up = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            up = true;
        } else if up {
            out.push(ch.to_ascii_uppercase());
            up = false;
        } else {
            out.push(ch);
        }
    }
    out
}

/* ── Primitive‑to‑Rust type mapping ───────────────────────────── */
fn rust_res_ty(res: &ApiResult) -> (&'static str, bool) {
    TypeRegistry.map_result_type(res)
}

fn rust_arg_ty(arg: &ApiArgument) -> (&'static str, bool) {
    TypeRegistry.map_argument_type(arg)
}

/// Generates type-safe client methods for Bitcoin RPC calls
pub struct RpcClientGenerator;

impl CodeGenerator for RpcClientGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut out = Vec::new();

        for m in methods {
            let mut code = String::new();

            /* doc comment */
            writeln!(code, "{}", doc_comment::format_doc_comment(&m.description)).unwrap();

            /* wrap inside impl RpcClient */
            writeln!(
                code,
                "use crate::transport::{{Transport, DefaultTransport}};"
            )
            .unwrap();
            writeln!(code, "impl crate::RpcClient {{").unwrap();

            /* inline Response struct if needed */
            let returns_unit = m.results.is_empty() || m.results[0].type_ == "null";
            if !returns_unit {
                writeln!(
                    code,
                    "    #[derive(Debug, serde::Deserialize)]\n    pub struct {}Response {{",
                    camel(&m.name)
                )
                .unwrap();
                for r in &m.results {
                    let (ty, opt) = rust_res_ty(r);
                    let field = camel_to_snake_case(&r.key_name);
                    if opt {
                        writeln!(code, "        pub {field}: Option<{ty}>,").unwrap();
                    } else {
                        writeln!(code, "        pub {field}: {ty},").unwrap();
                    }
                }
                writeln!(code, "    }}\n").unwrap();
            }

            /* function signature */
            write!(code, "    pub async fn {}(", camel_to_snake_case(&m.name)).unwrap();
            let mut params = Vec::new();
            for arg in m
                .arguments
                .iter()
                .filter(|a| a.required)
                .chain(m.arguments.iter().filter(|a| !a.required))
            {
                let (ty, opt) = rust_arg_ty(arg);
                let ident = camel_to_snake_case(&arg.names[0]);
                params.push(if opt {
                    format!("{ident}: Option<{ty}>")
                } else {
                    format!("{ident}: {ty}")
                });
            }
            let ret_ty = if returns_unit {
                "()".into()
            } else {
                format!("Self::{}Response", camel(&m.name))
            };
            writeln!(
                code,
                "&self, {}) -> Result<{}, crate::ClientError> {{",
                params.join(", "),
                ret_ty
            )
            .unwrap();

            /* build params vec */
            writeln!(code, "        let mut ps = Vec::new();").unwrap();
            for arg in &m.arguments {
                let id = camel_to_snake_case(&arg.names[0]);
                let (_, opt) = rust_arg_ty(arg);
                if opt {
                    writeln!(
                        code,
                        "        ps.push(match {id} {{ Some(v) => serde_json::to_value(v)?, None => serde_json::Value::Null }});"
                    )
                    .unwrap();
                } else {
                    writeln!(code, "        ps.push(serde_json::to_value({id})?);").unwrap();
                }
            }

            /* call + deserialize */
            writeln!(
                code,
                "        let raw = self.call_method(\"{}\", &ps).await?;",
                m.name
            )
            .unwrap();
            if returns_unit {
                writeln!(code, "        Ok(())").unwrap();
            } else {
                writeln!(code, "        Ok(serde_json::from_value(raw)?)").unwrap();
            }
            writeln!(code, "    }}\n}}\n").unwrap();

            /* file path */
            let path = format!("{}.rs", camel_to_snake_case(&m.name));
            out.push((path, code));
        }

        out
    }
}
