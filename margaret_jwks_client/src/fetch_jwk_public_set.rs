use reqwest::Client;
use reqwest::Response;
use url::Url;

use margaret_jwks_key_gen::jwk_public_set::JwkPublicSet;

use crate::jwks_client_error::JwksClientError;

pub async fn fetch_jwk_public_set(
    http_client: &Client,
    jwks_url: Url,
) -> Result<JwkPublicSet, JwksClientError> {
    http_client
        .get(jwks_url)
        .send()
        .await
        .and_then(Response::error_for_status)
        .map_err(JwksClientError::DocumentFetch)?
        .json::<JwkPublicSet>()
        .await
        .map_err(JwksClientError::DocumentFetch)
}
