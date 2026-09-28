use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn injects_a_dependency_imported_through_a_reexport() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/reexported_dependency");
    let source: String = generate_container_source("crate", &directory)
        .expect("the reexported dependency crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(
        source.contains("crate::Consumer::create(::std::sync::Arc::clone(&stores_store_store))")
    );
}
