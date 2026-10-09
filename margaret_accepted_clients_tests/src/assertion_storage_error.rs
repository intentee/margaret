use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssertionStorageError {
    #[error("the client assertion storage is unreachable")]
    Unreachable,
}
