use url::Url;

use margaret_registered_claims::issuer_identifier::IssuerIdentifier;

use crate::oidc_discovery_path::OIDC_DISCOVERY_PATH;

#[must_use]
pub fn oidc_discovery_url(issuer: &IssuerIdentifier) -> Url {
    let mut url = issuer.url().clone();
    let issuer_path = match url.path().strip_suffix('/') {
        Some(stripped) => stripped,
        None => url.path(),
    };
    let discovery_path = format!("{issuer_path}{OIDC_DISCOVERY_PATH}");

    url.set_path(&discovery_path);

    url
}
