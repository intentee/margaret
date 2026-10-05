use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;
use margaret_registered_claims::claims_merge_error::ClaimsMergeError;

#[derive(Debug, Error)]
pub enum JwksSecretStoreError {
    #[error("the access token claims could not be merged: {0}")]
    AccessTokenClaims(#[source] ClaimsMergeError),

    #[error("the id token could not be signed with the rsa key: {0}")]
    IdTokenSigning(#[source] JwksKeyError),
}
