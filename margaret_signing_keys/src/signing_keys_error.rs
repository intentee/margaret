use std::num::TryFromIntError;

use thiserror::Error;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;
use margaret_signing_keys_encryption::signing_keys_encryption_error::SigningKeysEncryptionError;

#[derive(Debug, Error)]
pub enum SigningKeysError {
    #[error("the signing keys cannot be created in the database: {0}")]
    Create(#[source] ActiveRecordError),

    #[error("the opened signing keys document is malformed: {0}")]
    DocumentMalformed(#[source] serde_json::Error),

    #[error("the stored signing keys cannot be opened: {0}")]
    DocumentOpen(#[source] SigningKeysEncryptionError),

    #[error("the signing keys cannot be sealed for storage: {0}")]
    DocumentSeal(#[source] SigningKeysEncryptionError),

    #[error("the signing keys cannot be serialized for storage: {0}")]
    DocumentSerialization(#[source] serde_json::Error),

    #[error(
        "the signing keys of generation {generation} exceed the generations the database keeps: {source}"
    )]
    GenerationOutOfRange {
        generation: SigningKeysGeneration,
        #[source]
        source: TryFromIntError,
    },

    #[error("the signing keys cannot be loaded from the database: {0}")]
    Load(#[source] ActiveRecordError),

    #[error("the signing keys cannot be replaced in the database: {0}")]
    Replace(#[source] ActiveRecordError),

    #[error(
        "the stored signing keys carry the generation {generation}, which no roll produces: {source}"
    )]
    StoredGenerationOutOfRange {
        generation: i64,
        #[source]
        source: TryFromIntError,
    },

    #[error("the stored signing keys carry a nonce of {length} bytes, which no seal produces")]
    StoredNonceLength { length: usize },
}
