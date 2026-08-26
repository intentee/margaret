use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn clones_a_serve_input_used_by_constructor_and_dependency() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/serve_input_constructor_and_dependency");
    let source: String = generate_container_source("crate", &directory)
        .expect("the mixed-consumption crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("crate::Parent::create("));
    assert!(source.contains("serve_input_0"));
    assert!(source.contains("serve_input_0.clone()"));
    assert!(source.contains("::std::sync::Arc::clone(&child)"));
}
