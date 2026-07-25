use thiserror::Error;

#[derive(Debug, Error)]
pub enum PostgresPoolError {
    #[error("the postgres connection URL is invalid: {source}")]
    InvalidConnectionUri {
        #[source]
        source: sqlx::Error,
    },

    #[error("the postgres max connections must be a non-zero unsigned integer: {source}")]
    InvalidMaxConnections {
        #[source]
        source: std::num::ParseIntError,
    },

    #[error("the postgres database is unreachable: {source}")]
    Unreachable {
        #[source]
        source: sqlx::Error,
    },
}
