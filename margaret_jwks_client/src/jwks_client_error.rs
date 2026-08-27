use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks document could not be fetched from the issuer: {0}")]
    DocumentFetch(#[source] reqwest::Error),

    #[error("the jwks endpoint could not be resolved: {0}")]
    EndpointResolution(#[source] anyhow::Error),

    #[error("the token claims could not be judged against the claims policy: {source:#}")]
    ClaimsAcceptance {
        #[source]
        source: anyhow::Error,
    },

    #[error("the published jwks could not be used to verify a token: {0}")]
    TokenVerification(#[source] JwksKeyError),
}
