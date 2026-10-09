use std::num::TryFromIntError;

use thiserror::Error;

use margaret_database::database_error::DatabaseError;
use margaret_jwks_keygen::signing_keys_generation::SigningKeysGeneration;

#[derive(Debug, Error)]
pub enum SigningKeysDatabaseError {
    #[error("the signing keys cannot be created in the database: {0}")]
    Create(#[source] tokio_postgres::Error),

    #[error(
        "the signing keys of generation {generation} exceed the generations the database keeps: {source}"
    )]
    GenerationOutOfRange {
        generation: SigningKeysGeneration,
        #[source]
        source: TryFromIntError,
    },

    #[error("the signing keys cannot be loaded from the database: {0}")]
    Load(#[source] tokio_postgres::Error),

    #[error("the stored signing keys are not a signing key set: {0}")]
    MalformedRow(#[source] tokio_postgres::Error),

    #[error("the signing keys cannot be replaced in the database: {0}")]
    Replace(#[source] tokio_postgres::Error),

    #[error(
        "the stored signing keys carry the generation {generation}, which no roll produces: {source}"
    )]
    StoredGenerationOutOfRange {
        generation: i64,
        #[source]
        source: TryFromIntError,
    },

    #[error("the database holding the signing keys is unavailable: {0}")]
    Unavailable(#[source] DatabaseError),
}
