use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwksSecretStorageSelectionError {
    #[error("--jwks-secret-file was provided, but the selected jwks secret storage is not 'file'")]
    FileOptionWithoutFileStorage,

    #[error("the 'file' jwks secret storage requires --jwks-secret-file to be provided")]
    FileStorageRequiresPath,
}
