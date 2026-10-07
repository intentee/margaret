use thiserror::Error;

use margaret_storage_uri::storage_uri_error::StorageUriError;

#[derive(Debug, Error)]
pub enum ProviderStateStorageUriError {
    #[error(
        "the openid connect provider state cannot be kept in a file; use 'memory:' or a 'postgres://' url"
    )]
    FileUnsupported,

    #[error("the openid connect provider state storage is not valid postgres options: {source}")]
    PostgresOptions {
        #[source]
        source: sqlx::Error,
    },

    #[error("the openid connect provider state storage uri is malformed: {0}")]
    StorageUri(#[from] StorageUriError),
}
