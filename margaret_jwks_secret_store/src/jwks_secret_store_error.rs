use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the claims to sign could not be serialized to json: {source}")]
    ClaimsSerialization {
        #[source]
        source: serde_json::Error,
    },

    #[error("the signing secret is not available yet")]
    SecretUnavailable,
}
