use url::Url;

use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

use crate::jwks_client_error::JwksClientError;

pub fn well_known_jwks_url(origin: &Url) -> Result<Url, JwksClientError> {
    origin
        .join(WELL_KNOWN_JWKS_PATH)
        .map_err(|source| JwksClientError::IssuerUrlNotABase {
            issuer_url: origin.to_string(),
            source,
        })
}
