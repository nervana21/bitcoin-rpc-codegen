// examples/compile_generated_v29/src/lib.rs

//! Top-level library for the generated v29 client & types.

pub mod client {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/generated/v29/client/mod.rs"
    ));
}

pub mod types {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/target/generated/v29/types/mod.rs"
    ));
}
