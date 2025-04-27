fn main() {
    // Only rebuild if source code changes, not resources.
    println!("cargo:rerun-if-changed=src/");
}
