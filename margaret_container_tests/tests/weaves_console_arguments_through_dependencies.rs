use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn weaves_console_arguments_through_dependencies() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let source: String = generate_container_source("crate", &directory)
        .expect("the propagation crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("pubasyncfnconstruct_config(console_argument_1:std::string::String,)"));
    assert!(source.contains("crate::Config::create(console_argument_1)"));
    assert!(
        source
            .contains("pubasyncfnconstruct_alpha_plugin(console_argument_0:std::string::String,)")
    );
    assert!(
        source.contains(
            "pubasyncfnconstruct_service(console_argument_1:std::string::String,console_argument_0:std::string::String,)"
        )
    );
    assert!(source.contains(
        "crate::Service::create(::std::sync::Arc::clone(&config),::std::sync::Arc::clone(&alpha_plugin),)"
    ));
}
