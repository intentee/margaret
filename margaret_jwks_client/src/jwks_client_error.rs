use reqwest::StatusCode;
use thiserror::Error;

use margaret_issuer_document_fetch::issuer_document_fetch_error::IssuerDocumentFetchError;
use margaret_jwks_keygen::jwks_key_error::JwksKeyError;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks document client could not be built: {source}")]
    ClientBuild {
        #[source]
        source: IssuerDocumentFetchError,
    },

    #[error("the jwks document is not a valid key set: {source}")]
    DocumentMalformed {
        #[source]
        source: serde_json::Error,
    },

    #[error("the issuer answered the jwks document request with status {status}")]
    DocumentStatus { status: StatusCode },

    #[error("the jwks document could not be transferred from the issuer: {source}")]
    DocumentTransport {
        #[source]
        source: reqwest::Error,
    },

    #[error("the jwks endpoint could not be resolved: {0}")]
    EndpointResolution(#[source] anyhow::Error),

    #[error("the token expiry could not be determined: {source:#}")]
    TokenExpiry {
        #[source]
        source: anyhow::Error,
    },

    #[error("the published jwks could not be used to verify a token: {0}")]
    TokenVerification(#[source] JwksKeyError),
}
