use std::sync::Arc;

use url::Url;

use margaret_oidc_discovery::advertised_endpoint::AdvertisedEndpoint;
use margaret_oidc_discovery::authorization_response_issuer::AuthorizationResponseIssuer;
use margaret_oidc_discovery::provider_metadata::ProviderMetadata;

fn localhost_url(path: &str) -> Url {
    Url::parse("https://localhost")
        .and_then(|origin| origin.join(path))
        .expect("the localhost endpoint is a url")
}

/// # Panics
///
/// Panics when a localhost endpoint is not a url.
#[must_use]
pub fn localhost_discovery_metadata(
    authorization_response_issuer: AuthorizationResponseIssuer,
    introspection_endpoint: AdvertisedEndpoint,
) -> Arc<ProviderMetadata> {
    Arc::new(ProviderMetadata {
        authorization_endpoint: AdvertisedEndpoint::Advertised(localhost_url("/authorize")),
        authorization_response_issuer,
        introspection_endpoint,
        jwks_uri: localhost_url("/jwks"),
        token_endpoint: AdvertisedEndpoint::Advertised(localhost_url("/token")),
        userinfo_endpoint: AdvertisedEndpoint::Advertised(localhost_url("/userinfo")),
    })
}
