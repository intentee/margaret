use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::served_endpoint::ServedEndpoint;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::published_endpoints::PublishedEndpoints;

fn located(issuer: &IssuerIdentifier, path: &str) -> &'static str {
    let mut url = issuer.url().clone();

    url.set_path(path);

    String::leak(url.to_string())
}

#[must_use]
pub fn fixture_endpoints(issuer: &IssuerIdentifier) -> PublishedEndpoints {
    let authorization = located(issuer, FIXTURE_ENDPOINT_PATHS.authorization);

    PublishedEndpoints {
        authorization,
        provider: ProviderEndpoints {
            authorization: ServedEndpoint::Served(authorization),
            introspection: ServedEndpoint::Served(located(
                issuer,
                FIXTURE_ENDPOINT_PATHS.introspection,
            )),
            issuer_origin: String::leak(issuer.url().origin().ascii_serialization()),
            jwks: located(issuer, FIXTURE_ENDPOINT_PATHS.jwks),
            revocation: ServedEndpoint::Served(located(issuer, FIXTURE_ENDPOINT_PATHS.revocation)),
            token: located(issuer, FIXTURE_ENDPOINT_PATHS.token),
            userinfo: ServedEndpoint::Served(located(issuer, FIXTURE_ENDPOINT_PATHS.userinfo)),
        },
    }
}
