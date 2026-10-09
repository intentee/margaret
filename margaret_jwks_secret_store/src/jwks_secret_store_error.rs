use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the id token could not be signed with the rsa key: {0}")]
    IdTokenSigning(#[source] JwksKeyError),
}
