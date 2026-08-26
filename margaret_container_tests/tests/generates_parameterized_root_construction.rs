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
        "pubfnconstruct_greet(super::construct_greet_arguments::ConstructGreetArguments{argument0:console_argument_0,argument1:console_argument_1,argument2:console_argument_2,}:super::construct_greet_arguments::ConstructGreetArguments,)"
    ));
    assert!(source.contains(
        "crate::Greet::create(::std::sync::Arc::clone(&english_greeter),console_argument_0,console_argument_1,console_argument_2,)"
    ));
}
