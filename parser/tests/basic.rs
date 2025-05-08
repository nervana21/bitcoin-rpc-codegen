// parser/tests/basic.rs

use parser::{DefaultHelpParser, HelpParser, ParserError};

static SAMPLE_HELP: &str = r#"
getblockchaininfo
Returns an object containing various state info regarding blockchain processing.

getnetworkinfo ( verbose )
Returns an object containing various state info regarding P2P networking.

stop
Immediately shuts down the server.
"#;

#[test]
fn parse_sample_blocks() {
    let parser = DefaultHelpParser;
    let methods = parser.parse(SAMPLE_HELP).expect("should parse");
    let names: Vec<_> = methods.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, &["getblockchaininfo", "getnetworkinfo", "stop"]);

    // Check raw text preservation
    assert!(methods[1].raw.contains("P2P networking"));
}

#[test]
fn empty_input_errors() {
    let parser = DefaultHelpParser;
    match parser.parse("") {
        Err(ParserError::NoMethods) => {}
        other => panic!("expected NoMethods, got {:?}", other),
    }
}

#[test]
fn ignores_category_headings() {
    let raw = r#"
== Foo ==
getfoo "arg"  Description of getfoo.

== Bar ==
getbar
    "#;

    let methods = DefaultHelpParser.parse(raw).expect("should parse methods");
    let names: Vec<_> = methods.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, &["getfoo", "getbar"]);
}
