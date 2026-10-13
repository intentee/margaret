use margaret_oidc_discovery::provider_endpoints::ProviderEndpoints;
use margaret_oidc_discovery::served_endpoint::ServedEndpoint;
use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::fixture_endpoint_paths::FIXTURE_ENDPOINT_PATHS;
use crate::published_endpoints::PublishedEndpoints;

fn located(issuer: &IssuerIdentifier, path: &str) -> String {
    let mut url = issuer.url().clone();

    url.set_path(path);

    url.to_string()
}

pub struct FixtureEndpointUrls {
    pub authorization: String,
    pub introspection: String,
    pub issuer_origin: String,
    pub jwks: String,
    pub revocation: String,
    pub token: String,
    pub userinfo: String,
}

impl FixtureEndpointUrls {
    #[must_use]
    pub fn of(issuer: &IssuerIdentifier) -> Self {
        Self {
            authorization: located(issuer, FIXTURE_ENDPOINT_PATHS.authorization),
            introspection: located(issuer, FIXTURE_ENDPOINT_PATHS.introspection),
            issuer_origin: issuer.url().origin().ascii_serialization(),
            jwks: located(issuer, FIXTURE_ENDPOINT_PATHS.jwks),
            revocation: located(issuer, FIXTURE_ENDPOINT_PATHS.revocation),
            token: located(issuer, FIXTURE_ENDPOINT_PATHS.token),
            userinfo: located(issuer, FIXTURE_ENDPOINT_PATHS.userinfo),
        }
    }

    #[must_use]
    pub fn published(&'static self) -> PublishedEndpoints {
        PublishedEndpoints {
            authorization: &self.authorization,
            provider: ProviderEndpoints {
                authorization: ServedEndpoint::Served(&self.authorization),
                introspection: ServedEndpoint::Served(&self.introspection),
                issuer_origin: &self.issuer_origin,
                jwks: &self.jwks,
                revocation: ServedEndpoint::Served(&self.revocation),
                token: &self.token,
                userinfo: ServedEndpoint::Served(&self.userinfo),
            },
        }
    }
}
