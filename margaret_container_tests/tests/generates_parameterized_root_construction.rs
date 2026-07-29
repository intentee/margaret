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
        "pub(crate)fnconstruct_greet(arguments:super::construct_greet_arguments::ConstructGreetArguments,)"
    ));
    assert!(source.contains(
        "crate::Greet::create(::std::sync::Arc::clone(&english_greeter),arguments.argument0,arguments.argument1,arguments.argument2,)"
    ));
}
