use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_token_signer_tests::token_issuance_declaration::TokenIssuanceDeclaration;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::provider_issuance::provider_issuance;

/// # Panics
///
/// Panics when the fixture discovery route is not served at the issuer.
#[must_use]
pub fn provider_endpoints_of_the_fixture() -> ProviderEndpoints {
    ProviderEndpoints::create(
        &TokenIssuanceDeclaration {
            issuance: provider_issuance(),
        },
        FIXTURE_ENDPOINT_PATHS,
    )
    .expect("the discovery route is served at the issuer")
}
