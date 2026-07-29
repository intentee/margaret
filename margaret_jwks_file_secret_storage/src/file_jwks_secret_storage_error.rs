use std::path::PathBuf;

use thiserror::Error;

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
        source: std::io::Error,
    },

    #[error("failed to serialize the jwks secret: {0}")]
    Serialize(#[source] serde_json::Error),

    #[error("failed to write the jwks secret file at '{}': {source}", .path.display())]
    Write {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
