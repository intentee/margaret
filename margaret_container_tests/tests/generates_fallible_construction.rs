use std::path::Path;

use margaret_container_tests::generate_container_source::generate_container_source;

fn fixture(name: &str) -> String {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);

    generate_container_source("crate", &directory)
        .expect("the fixture generates a container")
        .source()
        .split_whitespace()
        .collect()
}

#[test]
fn renders_a_fallible_root_builder_returning_a_result() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("pub(crate)fnconstruct_loader("));
    assert!(source.contains("::std::sync::Arc<crate::Loader>"));
    assert!(source.contains("margaret::framework::construction_error::ConstructionError"));
}

#[test]
fn wraps_a_fallible_constructor_through_the_framework_error_adapter() {
    let source = fixture("fallible_constructor");

    assert!(source.contains(
        "margaret::framework::construct_singleton::construct_singleton(\"crate::Loader\",crate::Loader::new(),)?"
    ));
    assert!(!source.contains("anyhow"));
}

#[test]
fn propagates_fallibility_to_an_explicitly_constructed_dependent() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("pub(crate)fnconstruct_consumer("));
    assert!(source.contains("::std::sync::Arc<crate::Consumer>"));
    assert!(source.contains("crate::Consumer::new("));
    assert!(source.contains("::std::sync::Arc::clone(&loader)"));
    assert!(source.contains("construct_singleton(\"crate::Consumer\""));
}

#[test]
fn renders_a_fallible_service_construction_that_depends_on_a_fallible_singleton() {
    let source = fixture("fallible_service");

    assert!(source.contains("pub(crate)fnconstruct_worker("));
    assert!(source.contains("::std::sync::Arc<crate::Worker>"));
    assert!(source.contains(
        "margaret::framework::construct_singleton::construct_singleton(\"crate::Worker\""
    ));
    assert!(source.contains("crate::Worker::new("));
    assert!(source.contains("::std::sync::Arc::clone(&loader)"));
}

#[test]
fn constructs_a_fieldless_singleton_without_wrapping() {
    let source = fixture("fieldless");

    assert!(source.contains("crate::UnitMarker"));
    assert!(!source.contains("::wrap("));
}
