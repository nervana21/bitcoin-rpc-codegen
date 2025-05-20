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
use crate::{doc_comment_generator, CodeGenerator};
use rpc_api::{ApiArgument, ApiMethod, ApiResult};
use std::fmt::Write as _;

/* ── Tiny case‑conversion helpers ─────────────────────────────── */
fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

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
    match res.type_.as_str() {
        "string" => ("String", false),
        "number" | "amount" | "numeric" => ("bitcoin::Amount", false),
        "boolean" => ("bool", false),
        "hex" => {
            let k = res.key_name.as_str();
            if k.contains("txid") {
                ("bitcoin::Txid", false)
            } else if k.contains("blockhash") {
                ("bitcoin::BlockHash", false)
            } else if k.contains("script") {
                ("bitcoin::Script", false)
            } else if k.contains("pubkey") {
                ("bitcoin::PublicKey", false)
            } else {
                ("String", false)
            }
        }
        "array" | "object" | "mixed" => ("serde_json::Value", false),
        "null" => ("()", false),
        _ => ("serde_json::Value", false),
    }
}

fn rust_arg_ty(arg: &ApiArgument) -> (&'static str, bool) {
    let dummy = ApiResult {
        key_name: arg.names[0].clone(),
        type_: arg.type_.clone(),
        description: String::new(),
        inner: vec![],
        optional: arg.optional,
    };
    let (ty, _) = rust_res_ty(&dummy);
    (ty, arg.optional)
}

/// Generates type-safe client methods for Bitcoin RPC calls
pub struct RpcClientGenerator;

impl CodeGenerator for RpcClientGenerator {
    fn generate(&self, methods: &[ApiMethod]) -> Vec<(String, String)> {
        let mut out = Vec::new();

        for m in methods {
            let mut code = String::new();

            /* doc comment */
            writeln!(
                code,
                "{}",
                doc_comment_generator::format_doc_comment(&m.description)
            )
            .unwrap();

            /* wrap inside impl RpcClient */
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
                    let field = snake(&r.key_name);
                    if opt {
                        writeln!(code, "        pub {}: Option<{}>,", field, ty).unwrap();
                    } else {
                        writeln!(code, "        pub {}: {},", field, ty).unwrap();
                    }
                }
                writeln!(code, "    }}\n").unwrap();
            }

            /* function signature */
            write!(code, "    pub async fn {}(", snake(&m.name)).unwrap();
            let mut params = Vec::new();
            for arg in m
                .arguments
                .iter()
                .filter(|a| !a.optional)
                .chain(m.arguments.iter().filter(|a| a.optional))
            {
                let (ty, opt) = rust_arg_ty(arg);
                let ident = snake(&arg.names[0]);
                params.push(if opt {
                    format!("{}: Option<{}>", ident, ty)
                } else {
                    format!("{}: {}", ident, ty)
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
                let id = snake(&arg.names[0]);
                let (_, opt) = rust_arg_ty(arg);
                if opt {
                    writeln!(
                        code,
                        "        ps.push(match {0} {{ Some(v) => serde_json::to_value(v)?, None => serde_json::Value::Null }});",
                        id
                    )
                    .unwrap();
                } else {
                    writeln!(code, "        ps.push(serde_json::to_value({})?);", id).unwrap();
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
            let path = format!("{}.rs", snake(&m.name));
            out.push((path, code));
        }

        out
    }
}
