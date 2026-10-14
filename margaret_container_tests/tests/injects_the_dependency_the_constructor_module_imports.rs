use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn injects_the_dependency_the_constructor_module_imports() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/constructor_in_another_module");
    let source: String = generate_container_source("crate", &directory)
        .expect("the crate constructing a singleton in another module renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("crate::Greeter::create(::std::sync::Arc::clone(&languages_english))"));
}
