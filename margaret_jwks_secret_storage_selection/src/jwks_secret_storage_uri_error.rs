use thiserror::Error;

use margaret_storage_uri::storage_uri_error::StorageUriError;

#[derive(Debug, Error)]
pub enum JwksSecretStorageUriError {
    #[error(
        "the jwks secret storage cannot be kept in postgres; use 'memory:' or a 'file:///' path"
    )]
    PostgresUnsupported,

    #[error("the jwks secret storage uri is malformed: {0}")]
    StorageUri(#[from] StorageUriError),
}
