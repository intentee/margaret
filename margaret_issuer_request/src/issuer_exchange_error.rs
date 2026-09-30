use thiserror::Error;

#[derive(Debug, Error)]
pub enum IssuerExchangeError {
    #[error("the issuer answered with more than {max_bytes} bytes")]
    Oversized { max_bytes: usize },

    #[error("the issuer could not be reached: {0}")]
    Transport(#[source] reqwest::Error),
}
