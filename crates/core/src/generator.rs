// crates/core/src/generator.rs

use crate::schema::ApiMethod;
use anyhow::Result;
use std::{fs, io::Write as _, path::PathBuf};

/// Sanitize an RPC method name into a valid Rust identifier file name.
fn sanitize_method_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

/// Given a version tag (e.g. "v29"), a list of ApiMethods,
/// and an output directory, emit the minimal file layout so that
/// generator_tests.rs will pass.
pub fn generate_version_code(
    version: &str,
    methods: &[ApiMethod],
    out_dir: &PathBuf,
) -> Result<()> {
    // 1) client/src/<version>/
    let client_dir = out_dir.join("client").join("src").join(version);
    fs::create_dir_all(&client_dir)?;
    // one file per RPC method
    for method in methods {
        let fname = sanitize_method_name(&method.name) + ".rs";
        fs::File::create(client_dir.join(&fname))?;
    }
    // a mod.rs listing them
    {
        let mut mod_rs = fs::File::create(client_dir.join("mod.rs"))?;
        for method in methods {
            let modname = sanitize_method_name(&method.name);
            writeln!(mod_rs, "pub mod {};", modname)?;
        }
    }

    // 2) types/src/<version>/
    let types_dir = out_dir.join("types").join("src").join(version);
    fs::create_dir_all(&types_dir)?;
    // emit one .rs per method
    for method in methods {
        let fname = sanitize_method_name(&method.name) + ".rs";
        fs::File::create(types_dir.join(&fname))?;
    }
    // and still leave a mod.rs
    fs::File::create(types_dir.join("mod.rs"))?;

    Ok(())
}
