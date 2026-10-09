use deadpool_postgres::BuildError;
use deadpool_postgres::PoolError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("the database url is malformed: {0}")]
    MalformedUrl(#[source] tokio_postgres::Error),

    #[error("the database connection pool cannot be built: {0}")]
    PoolBuild(#[source] BuildError),

    #[error("the database is unavailable: {0}")]
    Unavailable(#[source] PoolError),
}
