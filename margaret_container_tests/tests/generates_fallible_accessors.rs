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
fn renders_a_fallible_constructor_accessor_returning_a_result() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("pubasyncfnloader("));
    assert!(source.contains(
        "->::std::result::Result<std::sync::Arc<crate::Loader>,std::sync::Arc<margaret::framework::container_error::construction_error::ConstructionError,>,>"
    ));
}

#[test]
fn caches_a_fallible_construction_through_a_construction_slot_not_get_or_try_init() {
    let source = fixture("fallible_constructor");

    assert!(source.contains(
        "loader:margaret::framework::container_error::construction_slot::ConstructionSlot<crate::Loader,>"
    ));
    assert!(source.contains(
        "loader:margaret::framework::container_error::construction_slot::ConstructionSlot::default()"
    ));
    assert!(source.contains("self.loader.construct_once(\"crate::Loader\",asyncmove"));
    assert!(!source.contains("get_or_try_init"));
}

#[test]
fn wraps_a_fallible_constructor_through_the_framework_error_adapter() {
    let source = fixture("fallible_constructor");

    assert!(source.contains(
        "margaret::framework::container_error::construction_error::ConstructionError::wrap(\"crate::Loader\",crate::Loader::new(),)?"
    ));
    assert!(!source.contains("anyhow"));
}

#[test]
fn propagates_fallibility_to_an_explicitly_constructed_dependent() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("pubasyncfnconsumer("));
    assert!(source.contains(
        "->::std::result::Result<std::sync::Arc<crate::Consumer>,std::sync::Arc<margaret::framework::container_error::construction_error::ConstructionError,>,>"
    ));
    assert!(source.contains("crate::Consumer::new(self.loader().await?)"));
    assert!(source.contains("wrap(\"crate::Consumer\""));
}

#[test]
fn renders_a_fallible_service_construction_that_depends_on_a_fallible_singleton() {
    let source = fixture("fallible_service");

    assert!(source.contains("pubasyncfnworker("));
    assert!(source.contains(
        "->::std::result::Result<std::sync::Arc<crate::Worker>,std::sync::Arc<margaret::framework::container_error::construction_error::ConstructionError,>,>"
    ));
    assert!(source.contains(
        "margaret::framework::container_error::construction_error::ConstructionError::wrap(\"crate::Worker\",crate::Worker::new(self.loader().await?),)?"
    ));
}

#[test]
fn constructs_a_fieldless_singleton_through_the_slot_without_wrapping() {
    let source = fixture("fieldless");

    assert!(source.contains("construct_once"));
    assert!(!source.contains("get_or_try_init"));
    assert!(!source.contains("::wrap("));
}
