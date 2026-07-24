use std::path::Path;

use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

fn error(fixture: &str) -> ContainerError {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source(fixture, &directory).expect_err("the fixture must be rejected")
}

#[test]
fn rejects_a_provides_endpoint_on_a_non_struct() {
    assert!(matches!(
        error("endpoint_not_a_struct"),
        ContainerError::NotAnEndpointStruct { .. }
    ));
}

#[test]
fn rejects_an_endpoint_provider_that_is_also_a_service() {
    assert!(matches!(
        error("endpoint_conflicting_role"),
        ContainerError::ConflictingEndpointRole { .. }
    ));
}

#[test]
fn rejects_an_endpoint_provider_without_a_singleton() {
    assert!(matches!(
        error("endpoint_without_singleton"),
        ContainerError::EndpointProviderRequiresSingleton { .. }
    ));
}

#[test]
fn rejects_an_endpoint_provider_without_the_trait() {
    assert!(matches!(
        error("endpoint_missing_trait"),
        ContainerError::EndpointProviderMissingTrait { .. }
    ));
}

#[test]
fn rejects_an_endpoint_provider_with_fields_but_no_constructor() {
    assert!(matches!(
        error("endpoint_requires_constructor"),
        ContainerError::SingletonRequiresConstructor { .. }
    ));
}

#[test]
fn rejects_an_endpoint_provider_reference_to_an_unknown_tag() {
    assert!(
        error("endpoint_unknown_tag")
            .to_string()
            .contains("no endpoint provider declares")
    );
}

#[test]
fn rejects_an_endpoint_provider_reference_to_a_middleware_tag() {
    assert!(
        error("endpoint_wrong_kind")
            .to_string()
            .contains("is a middleware handler, not a endpoint provider")
    );
}

#[test]
fn rejects_a_malformed_endpoint_provider_reference() {
    assert!(
        error("endpoint_malformed_reference")
            .to_string()
            .contains("must reference exactly one tag")
    );
}

#[test]
fn rejects_an_unparseable_endpoint_provider_argument() {
    assert!(matches!(
        error("endpoint_unparseable_argument"),
        ContainerError::Index { .. }
    ));
}

#[test]
fn rejects_a_tag_declared_by_two_endpoint_providers() {
    assert!(
        error("endpoint_duplicate_tag")
            .to_string()
            .contains("declared more than once")
    );
}
