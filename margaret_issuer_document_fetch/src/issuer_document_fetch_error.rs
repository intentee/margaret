use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssuerDocumentFetchError {
    #[error("the issuer document client could not be built: {source}")]
    ClientBuild {
        #[source]
        source: reqwest::Error,
    },
}
