use url::Url;

use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

use crate::jwks_client_error::JwksClientError;

pub(crate) fn well_known_jwks_url(issuer_url: &Url) -> Result<Url, JwksClientError> {
    issuer_url
        .join(WELL_KNOWN_JWKS_PATH)
        .map_err(|source| JwksClientError::IssuerUrlNotABase {
            issuer_url: issuer_url.to_string(),
            source,
        })
}
