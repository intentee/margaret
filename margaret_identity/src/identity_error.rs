use thiserror::Error;

use margaret_jwks_roller::roller_error::RollerError;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("failed to maintain the JWKS signing keys: {source}")]
    KeyMaintenance {
        #[from]
        source: RollerError,
    },
}
