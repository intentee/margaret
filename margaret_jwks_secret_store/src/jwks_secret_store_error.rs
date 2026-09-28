use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the signing secret is not available yet")]
    SecretUnavailable,

    #[error("failed to sign claims with the current signing key: {source}")]
    Sign {
        #[source]
        source: JwksKeyError,
    },
}
