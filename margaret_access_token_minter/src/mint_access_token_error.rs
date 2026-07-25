use thiserror::Error;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[derive(Debug, Error)]
pub enum MintAccessTokenError {
    #[error("the request has no json body to mint from")]
    MissingBody,

    #[error("the request body is not a valid mint access token request: {source}")]
    MalformedRequest {
        #[source]
        source: serde_json::Error,
    },

    #[error("the access token could not be minted: {source}")]
    SecretStore {
        #[source]
        source: JwksSecretStoreError,
    },
}
