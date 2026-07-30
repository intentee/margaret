use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn names_container_fields_by_construction_position() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/full");
    let source: String = generate_container_source("full", &directory)
        .expect("the full fixture generates a container")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("provider0:::std::sync::Arc<"));
    assert!(source.contains("::std::sync::Arc::clone(&self.provider0)"));
    assert!(!source.contains("config:::std::sync::Arc<crate::Config>,"));
}
