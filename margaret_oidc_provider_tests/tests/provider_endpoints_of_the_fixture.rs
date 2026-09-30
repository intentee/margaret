use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::provider_issuance::provider_issuance;

pub fn provider_endpoints_of_the_fixture() -> ProviderEndpoints {
    ProviderEndpoints::create(&provider_issuance(), FIXTURE_ENDPOINT_PATHS)
        .expect("the discovery route is served at the issuer")
}
