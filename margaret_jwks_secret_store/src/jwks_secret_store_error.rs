use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_token_signer::token_signer_error::TokenSignerError;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the signing secret is not available yet")]
    SecretUnavailable,

    #[error("failed to sign claims with the current signing key: {source}")]
    Sign {
        #[source]
        source: JwksKeyError,
    },

    #[error("failed to verify a token against the signing secret: {source}")]
    Verify {
        #[source]
        source: JwksKeyError,
    },

    #[error("failed to mint an access token: {source}")]
    Mint {
        #[source]
        source: TokenSignerError,
    },
}
