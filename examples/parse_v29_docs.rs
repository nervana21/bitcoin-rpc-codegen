use anyhow::Result;
use bitcoin_rpc_codegen::parser::{ApiArgument, ApiMethod, ApiResult};
use regex::Regex;
use serde_json::{json, Map};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() -> Result<()> {
    let docs_dir = Path::new("resources/v29_docs");
    let mut commands = Map::new();

    for entry in fs::read_dir(docs_dir)? {
        let file = entry?;
        let method_name = file
            .path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let contents = fs::read_to_string(file.path())?;

        let description = extract_description(&contents);
        // let examples = extract_examples(&contents);
        let arguments = infer_arguments(&contents);
        let results = infer_results(&contents);

        let method_obj = ApiMethod {
            name: method_name.clone(),
            description,
            arguments,
            results,
        };

        commands.insert(method_name, json!([method_obj]));
    }

    let out_path = Path::new("resources/api_v29.json");
    let mut file = File::create(out_path)?;
    writeln!(
        file,
        "{}",
        serde_json::to_string_pretty(&json!({ "commands": commands }))?
    )?;

    println!("✅ Wrote structured schema to {}", out_path.display());
    Ok(())
}

fn extract_description(doc: &str) -> String {
    // Skip the first signature line and any following blank lines
    let lines = doc.lines().skip(1).skip_while(|l| l.trim().is_empty());
    let desc: String = lines
        .take_while(|l| {
            !l.trim_start().starts_with("Arguments:")
                && !l.trim_start().starts_with("Result:")
                && !l.trim_start().starts_with("Returns:")
                && !l.trim_start().starts_with("Examples:")
        })
        .collect::<Vec<_>>()
        .join("\n");

    if desc.trim().is_empty() {
        "<no description available>".to_string()
    } else {
        desc
    }
}

// fn extract_examples(doc: &str) -> String {
//     let mut lines = doc
//         .lines()
//         .skip_while(|l| !l.trim_start().starts_with("Examples:"));
//     lines.next(); // skip "Examples:"
//     lines.collect::<Vec<_>>().join("\n")
// }

fn infer_arguments(doc: &str) -> Vec<ApiArgument> {
    let mut capture = false;
    let mut args = vec![];

    let name_re = Regex::new(r#"^(\d+)\.\s+"?([a-zA-Z0-9_]+)"?\s+\(([^)]+)\)\s+(.*)$"#).unwrap();

    for line in doc.lines() {
        let line = line.trim();

        if line.starts_with("Arguments:") {
            capture = true;
            continue;
        }

        // Stop at next section header
        if (line.starts_with("Result")
            || line.starts_with("Returns")
            || line.starts_with("Examples:"))
            && capture
        {
            break;
        }

        if capture {
            if let Some(caps) = name_re.captures(line) {
                let name = caps[2].to_string();
                let typ = caps[3].to_string().to_lowercase();
                let desc = caps[4].to_string();
                args.push(ApiArgument {
                    names: vec![name],
                    type_: typ,
                    optional: caps[3].contains("optional"),
                    description: desc,
                });
            }
        }
    }

    args
}

fn infer_results(doc: &str) -> Vec<ApiResult> {
    let mut capture = false;
    let mut stack: Vec<(usize, Vec<ApiResult>)> = vec![(0, vec![])];
    let key_re = Regex::new(r#"^"([^"]+)"\s*:\s*(.*)$"#).unwrap();

    for line in doc.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("Result")
            || trimmed.starts_with("Returns")
            || trimmed.starts_with("Result =")
        {
            capture = true;
            continue;
        }

        if trimmed.starts_with("Arguments:")
            || trimmed.starts_with("Examples:")
            || trimmed.starts_with("1.")
            || trimmed.starts_with("0.")
        {
            break;
        }

        if !capture || trimmed.is_empty() {
            continue;
        }

        let depth = line.chars().take_while(|c| c.is_whitespace()).count();
        let mut type_ = "string";
        let mut desc = trimmed.to_string();
        let mut key_name = String::new();

        if let Some(cap) = key_re.captures(trimmed) {
            key_name = cap[1].trim().to_string();
            desc = cap[2].trim().to_string();
        }

        if desc.contains("(boolean)") {
            type_ = "boolean";
            desc = desc.replace("(boolean)", "").trim().to_string();
        } else if desc.contains("(string)") {
            type_ = "string";
            desc = desc.replace("(string)", "").trim().to_string();
        } else if desc.contains("(hex)") {
            type_ = "hex";
            desc = desc.replace("(hex)", "").trim().to_string();
        } else if desc.contains("(numeric)") {
            type_ = "number";
            desc = desc.replace("(numeric)", "").trim().to_string();
        } else if desc.contains("(json object)") {
            type_ = "object";
            desc = desc.replace("(json object)", "").trim().to_string();
        } else if desc.contains("(json null)") {
            type_ = "none";
            desc = desc.replace("(json null)", "").trim().to_string();
        }

        let result = ApiResult {
            type_: type_.to_string(),
            description: desc,
            key_name,
            inner: vec![],
        };

        while depth < stack.last().unwrap().0 {
            let (_child_depth, mut inner_results) = stack.pop().unwrap();
            if let Some((_parent_depth, parent)) = stack.last_mut() {
                if let Some(last) = parent.last_mut() {
                    last.inner.append(&mut inner_results);
                }
            }
        }

        stack.last_mut().unwrap().1.push(result);
        stack.push((depth, vec![]));
    }

    while stack.len() > 1 {
        let (_, mut inner_results) = stack.pop().unwrap();
        if let Some((_parent_depth, parent)) = stack.last_mut() {
            if let Some(last) = parent.last_mut() {
                last.inner.append(&mut inner_results);
            }
        }
    }

    let final_results = stack.pop().unwrap().1;
    if final_results.is_empty() {
        vec![ApiResult {
            type_: "none".to_string(),
            description: "".to_string(),
            key_name: "".to_string(),
            inner: vec![],
        }]
    } else {
        final_results
    }
}
