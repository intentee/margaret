use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

#[test]
fn reuses_one_dependency_across_sibling_singletons() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_single_fanout");
    let source: String = generate_container_source("crate", &directory)
        .expect("the fan-out crate renders")
        .source()
        .split_whitespace()
        .collect();

    assert!(source.contains("crate::Mapper::create(arguments.argument0)"));
    assert!(source.contains("crate::First::create(::std::sync::Arc::clone(&mapper))"));
    assert!(source.contains("crate::Second::create(::std::sync::Arc::clone(&mapper))"));
    assert!(source.contains("crate::Third::create(::std::sync::Arc::clone(&mapper))"));
}
