use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssuerExchangeError {
    #[error("the issuer could not be reached: {0}")]
    Transport(#[source] reqwest::Error),
}
