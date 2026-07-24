use thiserror::Error;

use margaret_endpoint::endpoint_error::EndpointError;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks document could not be fetched from the issuer: {0}")]
    DocumentFetch(#[source] reqwest::Error),

    #[error("the jwks endpoint could not be resolved: {0}")]
    EndpointResolution(#[source] EndpointError),

    #[error("the jwks document has not been fetched from the issuer yet")]
    NotReady,

    #[error("the token expired before it was verified")]
    TokenExpired,

    #[error("the token could not be verified against the published jwks: {0}")]
    TokenVerification(#[source] JwksKeyError),
}
