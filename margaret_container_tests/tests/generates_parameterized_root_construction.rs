use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn generates_parameterized_root_construction_for_console_arguments() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_arguments");
    let source: String = generate_container_source("crate", &directory)
        .expect("the console argument crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains(
        "pubasyncfnconstruct_greet(console_argument_0:std::string::String,console_argument_1:::std::option::Option<std::string::String>,console_argument_2:bool,)"
    ));
    assert!(source.contains(
        "crate::Greet::create(::std::sync::Arc::clone(&english_greeter),console_argument_0,console_argument_1,console_argument_2,)"
    ));
}
