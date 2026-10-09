use thiserror::Error;

#[derive(Debug, Error)]
pub enum SubjectTokenExchangeError {
    #[error("the subject token exchanger failed: {source:#}")]
    ExchangerFailed {
        #[source]
        source: anyhow::Error,
    },
}
