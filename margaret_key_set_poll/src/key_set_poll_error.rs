use thiserror::Error;

use margaret_issuer_document_fetch::issuer_document_fetch_error::IssuerDocumentFetchError;

#[derive(Debug, Error)]
pub enum KeySetPollError {
    #[error("the issuer document client could not be built: {source}")]
    ClientBuild {
        #[source]
        source: IssuerDocumentFetchError,
    },
}
