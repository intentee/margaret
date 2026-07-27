use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn wires_dependencies_through_linear_construction() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let generated = generate_container_source("crate", &directory)
        .expect("the full fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("crate::EnglishGreeter::new(::std::sync::Arc::clone(&config))"));
    assert!(source.contains(
        "crate::App::new(::std::sync::Arc::clone(&english_greeter),::std::sync::Arc::clone(&config),)"
    ));
    assert!(!source.contains("construct_once"));
    assert!(!source.contains("ConstructionSlot"));
}
