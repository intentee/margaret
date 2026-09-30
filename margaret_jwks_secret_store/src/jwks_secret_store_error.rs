use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the claims to sign could not be serialized to json: {source}")]
    ClaimsSerialization {
        #[source]
        source: serde_json::Error,
    },

    #[error("the id token could not be signed with the rsa key: {source}")]
    IdTokenSigning {
        #[source]
        source: JwksKeyError,
    },

    #[error("the signing secret is not available yet")]
    SecretUnavailable,
}
