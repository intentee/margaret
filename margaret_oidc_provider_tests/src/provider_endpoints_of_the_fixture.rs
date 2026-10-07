use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;

use crate::fixture_endpoints::fixture_endpoints;
use crate::provider_issuance::provider_issuance;

/// # Panics
///
/// Panics when the fixture issuer is not an https url.
#[must_use]
pub fn provider_endpoints_of_the_fixture() -> ProviderEndpoints {
    fixture_endpoints(
        &provider_issuance()
            .issuer
            .parse()
            .expect("the fixture issuer is an https url"),
    )
}
