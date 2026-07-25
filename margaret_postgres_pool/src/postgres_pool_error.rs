use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresPoolError {
    #[error("the postgres connection URL is invalid: {source}")]
    InvalidConnectionUri {
        #[source]
        source: sqlx::Error,
    },
}
