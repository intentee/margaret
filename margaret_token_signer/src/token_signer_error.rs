use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum TokenSignerError {
    #[error("failed to sign a token with the current signing key: {source}")]
    Signing { source: JwksKeyError },

    #[error("failed to verify a refresh token against the configured signing keys: {source}")]
    Verification { source: JwksKeyError },
}
