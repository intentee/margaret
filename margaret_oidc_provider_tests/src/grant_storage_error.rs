use thiserror::Error;

use crate::grant_operation::GrantOperation;

#[derive(Debug, Error)]
pub enum GrantStorageError {
    #[error("the authorization grant storage is unreachable during {operation:?}")]
    Unreachable { operation: GrantOperation },
}
