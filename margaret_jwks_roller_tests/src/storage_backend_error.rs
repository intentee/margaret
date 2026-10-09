use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageBackendError {
    #[error("the jwks secret backend is unreachable")]
    Unreachable,
}
