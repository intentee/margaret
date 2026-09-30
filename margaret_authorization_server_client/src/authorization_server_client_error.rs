use thiserror::Error;

use margaret_jwks_secret_store::jwks_secret_store_error::JwksSecretStoreError;

#[derive(Debug, Error)]
pub enum AuthorizationServerClientError {
    #[error("the client assertion could not be signed: {0}")]
    AssertionSigning(#[source] JwksSecretStoreError),
}
