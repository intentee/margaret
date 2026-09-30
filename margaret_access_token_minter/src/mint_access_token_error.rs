use thiserror::Error;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[derive(Debug, Error)]
pub enum MintAccessTokenError {
    #[error("the access token could not be minted: {source}")]
    SecretStore {
        #[source]
        source: JwksSecretStoreError,
    },
}
