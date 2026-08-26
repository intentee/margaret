use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn moves_a_copy_serve_input_shared_by_dependencies() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/serve_input_copy_fanout");
    let source: String = generate_container_source("crate", &directory)
        .expect("the copy fan-out crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("crate::Parent::create("));
    assert!(source.contains("serve_input_0"));
    assert!(source.contains("serve_input_1"));
    assert!(source.contains("serve_input_2"));
    assert!(source.contains("::std::sync::Arc::clone(&child)"));
    assert!(!source.contains(".clone()"));
}
