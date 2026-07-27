use thiserror::Error;

use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks endpoint must be an HTTPS URL without credentials, query, or fragment")]
    InvalidEndpoint,

    #[error("the jwks document could not be fetched from the issuer: {0}")]
    DocumentFetch(#[source] reqwest::Error),

    #[error("the jwks endpoint returned a response without application/jwk-set+json")]
    InvalidContentType,

    #[error("the jwks document exceeds the {limit} byte limit")]
    DocumentTooLarge { limit: usize },

    #[error("the jwks document is not valid: {source}")]
    InvalidDocument {
        #[source]
        source: serde_json::Error,
    },

    #[error("the jwks document contains duplicate key id '{kid}'")]
    DuplicateKeyId { kid: String },

    #[error("the jwks endpoint could not be resolved: {0}")]
    EndpointResolution(#[source] anyhow::Error),

    #[error("the token expiry could not be determined: {source:#}")]
    TokenExpiry {
        #[source]
        source: anyhow::Error,
    },

    #[error("the jwks document has not been fetched from the issuer yet")]
    NotReady,

    #[error("the token could not be verified against the published jwks: {0}")]
    TokenVerification(#[source] JwksKeyError),
}
