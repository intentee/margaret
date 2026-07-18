use thiserror::Error;

use margaret_jwks_roller::roller_error::RollerError;

#[derive(Debug, Error)]
pub enum JwksRollerServerError {
    #[error("the published jwks document could not be serialized: {0}")]
    DocumentSerialization(#[source] serde_json::Error),

    #[error("the jwks secret could not be rolled: {0}")]
    SecretRoll(#[source] RollerError),
}
