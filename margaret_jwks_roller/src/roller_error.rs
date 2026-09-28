use std::error;

use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum RollerError {
    #[error("failed to generate a jwks signing key: {0}")]
    KeyGeneration(#[source] JwksKeyError),

    #[error("failed to load the persisted jwks secret: {source}")]
    SecretLoad {
        #[source]
        source: Box<dyn error::Error + Send + Sync>,
    },

    #[error("failed to persist the rolled jwks secret: {source}")]
    SecretPersist {
        #[source]
        source: Box<dyn error::Error + Send + Sync>,
    },
}
