use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum TokenSignerError {
    #[error("failed to verify the refresh token against the signing secret: {source}")]
    RefreshTokenVerification {
        #[source]
        source: JwksKeyError,
    },

    #[error("failed to sign a token with the current signing key: {source}")]
    Signing {
        #[source]
        source: JwksKeyError,
    },
}
