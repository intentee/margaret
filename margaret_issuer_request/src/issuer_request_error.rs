use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssuerRequestError {
    #[error("the issuer request client could not be built: {source}")]
    ClientBuild {
        #[source]
        source: reqwest::Error,
    },
}
