use std::io;
use std::path::PathBuf;

use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum FileJwksSecretStorageError {
    #[error("failed to deserialize the jwks secret file at '{}': {source}", .path.display())]
    Deserialize {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("the jwks secret path '{}' is a filesystem root and has no parent directory", .path.display())]
    PathHasNoParentDirectory { path: PathBuf },

    #[error("failed to read the jwks secret file at '{}': {source}", .path.display())]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error("failed to restore the jwks secret read from '{}': {source}", .path.display())]
    Restore {
        path: PathBuf,
        #[source]
        source: JwksKeyError,
    },

    #[error("failed to serialize the jwks secret: {0}")]
    Serialize(#[source] serde_json::Error),

    #[error("failed to write the jwks secret file at '{}': {source}", .path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
}
