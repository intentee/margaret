use std::io::Error as IoError;

use thiserror::Error;

use margaret_jwks_key_gen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum RollerError {
    #[error("failed to deserialize the jwks secret at '{path}': {source}")]
    Deserialize {
        path: String,
        source: serde_json::Error,
    },

    #[error("failed to generate a jwks signing key: {0}")]
    KeyGeneration(#[source] JwksKeyError),

    #[error("failed to read the jwks secret at '{path}': {source}")]
    Read { path: String, source: IoError },

    #[error("failed to serialize the jwks secret: {source}")]
    Serialize { source: serde_json::Error },

    #[error("failed to write the jwks secret to '{path}': {source}")]
    Write { path: String, source: IoError },
}
