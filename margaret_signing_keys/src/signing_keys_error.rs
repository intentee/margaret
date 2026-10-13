use std::num::TryFromIntError;

use thiserror::Error;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

#[derive(Debug, Error)]
pub enum SigningKeysError {
    #[error("the signing keys cannot be created in the database: {0}")]
    Create(#[source] ActiveRecordError),

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
}
