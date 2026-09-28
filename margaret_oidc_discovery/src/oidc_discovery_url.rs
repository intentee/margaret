use url::Url;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

const WELL_KNOWN_OPENID_CONFIGURATION: &str = "/.well-known/openid-configuration";

#[must_use]
pub fn oidc_discovery_url(issuer: &IssuerIdentifier) -> Url {
    let mut url = issuer.url().clone();
    let issuer_path = match url.path().strip_suffix('/') {
        Some(stripped) => stripped,
        None => url.path(),
    };
    let discovery_path = format!("{issuer_path}{WELL_KNOWN_OPENID_CONFIGURATION}");

    url.set_path(&discovery_path);

    url
}
