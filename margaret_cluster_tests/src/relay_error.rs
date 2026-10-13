use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelayError {
    #[error("the relay cannot connect to its backend: {0}")]
    BackendConnect(#[source] io::Error),

    #[error("the relay cannot complete the TLS handshake: {0}")]
    Handshake(#[source] io::Error),

    #[error("the relay has no admitted backend")]
    NoAdmittedBackend,

    #[error("the relay cannot transfer the connection: {0}")]
    Transfer(#[source] io::Error),
}
