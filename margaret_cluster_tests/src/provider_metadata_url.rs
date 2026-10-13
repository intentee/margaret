use url::Url;

use margaret::framework::oidc_discovery::oidc_discovery_path::OIDC_DISCOVERY_PATH;

/// # Panics
///
/// Panics when the discovery path does not join the identity URL.
#[must_use]
pub fn provider_metadata_url(identity: &Url) -> Url {
    identity
        .join(OIDC_DISCOVERY_PATH)
        .expect("the provider metadata URL joins")
}
