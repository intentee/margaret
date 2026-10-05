use margaret_oidc_provider::provider_endpoint_paths::ProviderEndpointPaths;
use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_oidc_provider_tests::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use margaret_oidc_provider_tests::provider_issuance::provider_issuance;
use margaret_token_signer_tests::token_issuance_declaration::TokenIssuanceDeclaration;

#[test]
fn rejects_a_discovery_route_outside_the_issuer() {
    let Err(rejection) = ProviderEndpoints::create(
        &TokenIssuanceDeclaration {
            issuance: provider_issuance(),
        },
        ProviderEndpointPaths {
            discovery: "/openid-configuration",
            ..FIXTURE_ENDPOINT_PATHS
        },
    ) else {
        panic!("the discovery route must be served at the issuer");
    };

    assert_eq!(
        rejection.to_string(),
        "the discovery route '/openid-configuration' is not served at the discovery location 'https://localhost/.well-known/openid-configuration' of the issuer"
    );
}
