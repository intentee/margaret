use std::collections::HashMap;

use margaret_oidc_provider::introspection_endpoint::IntrospectionEndpoint;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_validation::validate::validate;

use crate::request_authorized_by::request_authorized_by;
use crate::unrolled_provider::UnrolledProvider;

#[test]
fn reports_an_introspection_before_the_first_roll() {
    let provider = UnrolledProvider::create();
    let endpoint = IntrospectionEndpoint::create(provider.clients, provider.secret_store);

    assert!(matches!(
        endpoint.respond(
            &request_authorized_by("Basic cG9ydGFsOnBvcnRhbC1zZWNyZXQ="),
            validate(&HashMap::from([("token".to_string(), "a.b.c".to_string())])),
        ),
        Err(ProviderError::Signing(_))
    ));
}
