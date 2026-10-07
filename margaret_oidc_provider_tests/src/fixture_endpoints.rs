use margaret_oidc_provider::provider_endpoints::ProviderEndpoints;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;

fn located(issuer: &IssuerIdentifier, path: &str) -> &'static str {
    let mut url = issuer.url().clone();

    url.set_path(path);

    String::leak(url.to_string())
}

#[must_use]
pub fn fixture_endpoints(issuer: &IssuerIdentifier) -> ProviderEndpoints {
    ProviderEndpoints {
        authorization: located(issuer, FIXTURE_ENDPOINT_PATHS.authorization),
        introspection: located(issuer, FIXTURE_ENDPOINT_PATHS.introspection),
        issuer_origin: String::leak(issuer.url().origin().ascii_serialization()),
        jwks: located(issuer, FIXTURE_ENDPOINT_PATHS.jwks),
        revocation: located(issuer, FIXTURE_ENDPOINT_PATHS.revocation),
        token: located(issuer, FIXTURE_ENDPOINT_PATHS.token),
        userinfo: located(issuer, FIXTURE_ENDPOINT_PATHS.userinfo),
    }
}
