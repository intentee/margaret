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
        "->Result<std::sync::Arc<crate::Loader>,margaret::framework::container_error::construction_error::ConstructionError,>"
    ));
    assert!(source.contains(".get_or_try_init(||asyncmove"));
}

#[test]
fn maps_a_failed_constructor_into_the_user_error_variant() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("letoutcome:::anyhow::Result<crate::Loader>=crate::Loader::new();"));
    assert!(source.contains(
        "outcome.map_err(|source|margaret::framework::container_error::construction_error::ConstructionError::user_error(\"crate::Loader\",source,))?"
    ));
}

#[test]
fn propagates_fallibility_to_an_infallible_dependent() {
    let source = fixture("fallible_constructor");

    assert!(source.contains("pubasyncfnconsumer("));
    assert!(source.contains(
        "->Result<std::sync::Arc<crate::Consumer>,margaret::framework::container_error::construction_error::ConstructionError,>"
    ));
    assert!(source.contains("crate::Consumer::new(self.loader().await?)"));
    assert!(!source.contains("user_error(\"crate::Consumer\""));
}

#[test]
fn renders_a_fallible_service_construction_that_depends_on_a_fallible_singleton() {
    let source = fixture("fallible_service");

    assert!(source.contains("pubasyncfnworker("));
    assert!(source.contains(
        "->Result<std::sync::Arc<crate::Worker>,margaret::framework::container_error::construction_error::ConstructionError,>"
    ));
    assert!(source.contains(
        "letoutcome:::anyhow::Result<crate::Worker>=crate::Worker::new(self.loader().await?,);"
    ));
    assert!(source.contains(
        "outcome.map_err(|source|margaret::framework::container_error::construction_error::ConstructionError::user_error(\"crate::Worker\",source,))?"
    ));
}

#[test]
fn leaves_infallible_accessors_unchanged() {
    let source = fixture("services");

    assert!(!source.contains("get_or_try_init"));
    assert!(!source.contains("ConstructionError"));
}
