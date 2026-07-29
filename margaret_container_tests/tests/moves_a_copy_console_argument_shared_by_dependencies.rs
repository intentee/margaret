use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn moves_a_copy_console_argument_shared_by_dependencies() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_copy_fanout");
    let source: String = generate_container_source("crate", &directory)
        .expect("the copy fan-out crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("crate::Parent::create("));
    assert!(source.contains("arguments.argument0"));
    assert!(source.contains("::std::sync::Arc::clone(&child)"));
    assert!(!source.contains("arguments.argument0.clone()"));
}
