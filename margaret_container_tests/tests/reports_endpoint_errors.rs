use std::path::Path;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_container_tests::generate_container_source::generate_container_source;

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
        ContainerError::DeclarationNotAStruct {
            attribute: "provides_jwks_endpoint",
            ..
        }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_that_is_also_a_service() {
    assert!(matches!(
        error("endpoint_conflicting_role"),
        ContainerError::ConflictingDeclarationRole {
            attribute: "provides_jwks_endpoint",
            ..
        }
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_without_a_singleton() {
    assert!(matches!(
        error("endpoint_without_singleton"),
        ContainerError::DeclarationRequiresSingleton {
            attribute: "provides_jwks_endpoint",
            ..
        }
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
        ContainerError::DeclarationMissingTrait { required, .. }
            if required == "margaret::framework::jwks_endpoint::provides_endpoint::ProvidesEndpoint"
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_without_the_token_trust() {
    assert!(matches!(
        error("endpoint_missing_token_trust"),
        ContainerError::DeclarationMissingTrait { required, .. }
            if required == "margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust"
    ));
}

#[test]
fn rejects_a_jwks_endpoint_provider_with_fields_but_no_constructor() {
    assert!(matches!(
        error("endpoint_requires_constructor"),
        ContainerError::SingletonRequiresConstructor { .. }
    ));
}
