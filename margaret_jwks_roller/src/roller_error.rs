use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_jwks_keygen::signing_curve::SigningCurve;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys::signing_keys_error::SigningKeysError;

#[derive(Debug, Error)]
pub enum RollerError {
    #[error("the store reported signing keys created by another instance, but a load found none")]
    CreatedKeysNotObserved,

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

    #[error(
        "the stored signing keys of generation {generation} differ from the held ones of the same generation"
    )]
    GenerationForked { generation: SigningKeysGeneration },

    #[error("the stored signing keys regressed from generation {held} to generation {stored}")]
    GenerationRegressed {
        held: SigningKeysGeneration,
        stored: SigningKeysGeneration,
    },

    #[error("failed to generate a jwks signing key: {0}")]
    KeyGeneration(#[source] JwksKeyError),

    #[error("failed to roll the jwks signing keys: {0}")]
    KeyRoll(#[source] JwksKeyError),

    #[error("the application could not create its first signing keys: {source}")]
    SecretCreate {
        #[source]
        source: SigningKeysError,
    },

    #[error("the application could not load its stored signing keys: {source}")]
    SecretLoad {
        #[source]
        source: SigningKeysError,
    },

    #[error("the application could not replace its stored signing keys: {source}")]
    SecretReplace {
        #[source]
        source: SigningKeysError,
    },

    #[error("the signing keys of generation {generation} vanished from the store")]
    StoredKeysVanished { generation: SigningKeysGeneration },

    #[error(
        "the store superseded the signing keys of generation {expected}, but a load found generation {observed}"
    )]
    SupersedingKeysNotObserved {
        expected: SigningKeysGeneration,
        observed: SigningKeysGeneration,
    },
}
