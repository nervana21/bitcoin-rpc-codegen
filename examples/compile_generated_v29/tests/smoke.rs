#[test]
fn smoke() {
    // This test will pass if the example compiles and the binary is callable.
    assert_eq!(
        0,
        std::process::Command::new(env!("CARGO_BIN_EXE_compile_generated_v29"))
            .status()
            .unwrap()
            .code()
            .unwrap()
    );
}
