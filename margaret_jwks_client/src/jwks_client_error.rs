use reqwest::StatusCode;
use thiserror::Error;

use margaret_issuer_document_fetch::issuer_document_fetch_error::IssuerDocumentFetchError;
use margaret_jws_verification::key_set_rejection::KeySetRejection;

#[derive(Debug, Error)]
pub enum JwksClientError {
    #[error("the jwks document client could not be built: {source}")]
    ClientBuild {
        #[source]
        source: IssuerDocumentFetchError,
    },

    #[error("the jwks document is rejected: {rejection}")]
    DocumentRejected { rejection: KeySetRejection },

    #[error("the issuer answered the jwks document request with status {status}")]
    DocumentStatus { status: StatusCode },

    #[error("the jwks document could not be transferred from the issuer: {source}")]
    DocumentTransport {
        #[source]
        source: reqwest::Error,
    },

    #[error("the jwks endpoint could not be resolved: {0}")]
    EndpointResolution(#[source] anyhow::Error),
}
