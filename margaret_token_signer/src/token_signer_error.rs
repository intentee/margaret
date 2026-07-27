use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum TokenSignerError {
    #[error("failed to determine whether the refresh token is expired: {source:#}")]
    Expiry {
        #[source]
        source: anyhow::Error,
    },

    #[error("the refresh token is expired")]
    ExpiredRefreshToken,

    #[error("the refresh token signature does not match any known signing key")]
    InvalidRefreshToken,

    #[error("failed to sign a token with the current signing key: {source}")]
    Signing { source: JwksKeyError },

    #[error("the refresh token could not be parsed or verified: {source}")]
    UnverifiableRefreshToken { source: JwksKeyError },
}
