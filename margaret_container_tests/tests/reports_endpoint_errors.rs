use std::path::Path;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;
use margaret_tag_codegen::tag_error::TagError;

fn error(fixture: &str) -> ContainerError {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(fixture);

    generate_container_source(fixture, &directory).expect_err("the fixture must be rejected")
}

#[test]
fn rejects_a_provides_jwks_endpoint_on_a_non_struct() {
    assert!(matches!(
        error("endpoint_not_a_struct"),
        ContainerError::NotAnEndpointStruct { .. }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_that_is_also_a_service() {
    assert!(matches!(
        error("endpoint_conflicting_role"),
        ContainerError::ConflictingEndpointRole { .. }
    ));
}

#[test]
fn rejects_two_jwks_endpoint_providers_sharing_a_tag() {
    assert!(matches!(
        error("endpoint_shared_tag"),
        ContainerError::Tag {
            source: TagError::DuplicateTag { .. }
        }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_without_a_singleton() {
    assert!(matches!(
        error("endpoint_without_singleton"),
        ContainerError::EndpointProviderRequiresSingleton { .. }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_that_carries_singleton_arguments() {
    assert!(matches!(
        error("endpoint_singleton_with_arguments"),
        ContainerError::SingletonHasArguments { .. }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_with_malformed_singleton_arguments() {
    assert!(matches!(
        error("endpoint_malformed_singleton_arguments"),
        ContainerError::Index {
            source: AttributeError::Arguments(AttributeArgumentsError::Malformed { .. })
        }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_without_the_trait() {
    assert!(matches!(
        error("endpoint_missing_trait"),
        ContainerError::EndpointProviderMissingTrait { .. }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_with_fields_but_no_constructor() {
    assert!(matches!(
        error("endpoint_requires_constructor"),
        ContainerError::SingletonRequiresConstructor { .. }
    ));
}

#[test]
fn propagates_an_unparseable_jwks_store_on_an_endpoint_constructor() {
    assert!(matches!(
        error("endpoint_unparseable_jwks_store"),
        ContainerError::Index { .. }
    ));
}
