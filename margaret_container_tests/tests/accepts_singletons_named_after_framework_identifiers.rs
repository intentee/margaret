use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn accepts_singletons_named_after_framework_identifiers() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/framework_named_singletons");
    let source: String = generate_container_source("framework_named_singletons", &directory)
        .expect("singletons named after framework identifiers are accepted without disambiguation")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("pub(crate)fnconstruct_build("));
    assert!(source.contains("pub(crate)fnconstruct_container("));
    assert!(source.contains("pub(crate)fnconstruct_routes("));
    assert!(source.contains("pub(crate)fnserve("));
    assert!(!source.contains("build_2"));
    assert!(!source.contains("container_2"));
    assert!(!source.contains("routes_2"));
}
