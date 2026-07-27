use std::path::Path;

use margaret_container::container_error::ContainerError;
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
fn rejects_injecting_a_non_singleton_role() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/injecting_a_service");
    let error = generate_container_source("injecting_a_service", &directory)
        .expect_err("injecting a non-singleton service must be rejected");

    assert!(matches!(error, ContainerError::MissingProvider { .. }));
}

#[test]
fn rejects_a_construction_depending_on_a_non_singleton() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/service_with_a_non_singleton_dependency");
    let error = generate_container_source("service_with_a_non_singleton_dependency", &directory)
        .expect_err("a service depending on another non-singleton service must be rejected");

    assert!(matches!(error, ContainerError::MissingProvider { .. }));
}

#[test]
fn rejects_a_construction_with_fields_but_no_constructor() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/service_with_fields_but_no_constructor");
    let error = generate_container_source("service_with_fields_but_no_constructor", &directory)
        .expect_err("a field-bearing construction without a constructor must be rejected");

    assert!(matches!(
        error,
        ContainerError::SingletonRequiresConstructor { .. }
    ));
}

#[test]
fn injects_a_singleton_that_also_carries_a_role() {
    let source = fixture("singleton_ticker");

    assert!(source.contains("crate::Reader::new(::std::sync::Arc::clone(&roller))"));
}
