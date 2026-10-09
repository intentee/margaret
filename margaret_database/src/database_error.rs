use deadpool_postgres::BuildError;
use deadpool_postgres::PoolError;
use thiserror::Error;

use crate::isolation::Isolation;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("the database does not begin a {isolation:?} transaction: {source}")]
    Begin {
        isolation: Isolation,
        #[source]
        source: tokio_postgres::Error,
    },

    #[error("the database does not commit the transaction: {0}")]
    Commit(#[source] tokio_postgres::Error),

    #[error("the database url is malformed: {0}")]
    MalformedUrl(#[source] tokio_postgres::Error),

    #[error("the database connection pool cannot be built: {0}")]
    PoolBuild(#[source] BuildError),

    #[error("the database does not roll the transaction back: {0}")]
    Rollback(#[source] tokio_postgres::Error),

    #[error("the database does not execute the statement: {0}")]
    StatementExecution(#[source] tokio_postgres::Error),

    #[error("the database does not prepare the statement: {0}")]
    StatementPreparation(#[source] tokio_postgres::Error),

    #[error("the database is unavailable: {0}")]
    Unavailable(#[source] PoolError),
}
