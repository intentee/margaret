use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;

#[derive(Debug, Error)]
pub enum RollerError {
    #[error("the stored signing keys use {stored:?} where Margaret signs with {pinned:?}")]
    CurveMismatch {
        pinned: SigningCurve,
        stored: SigningCurve,
    },

    #[error("the stored signing keys document is malformed: {source}")]
    DocumentMalformed {
        #[source]
        source: serde_json::Error,
    },

    #[error("the stored signing keys cannot be restored: {source}")]
    DocumentRestore {
        #[source]
        source: JwksKeyError,
    },

    #[error("the signing keys cannot be serialized for storage: {0}")]
    DocumentSerialization(#[source] serde_json::Error),

    #[error("failed to generate a jwks signing key: {0}")]
    KeyGeneration(#[source] JwksKeyError),

    #[error("the application could not load its stored signing keys: {source}")]
    SecretLoad {
        #[source]
        source: anyhow::Error,
    },

    #[error("the application could not store the rolled signing keys: {source}")]
    SecretPersist {
        #[source]
        source: anyhow::Error,
    },
}
