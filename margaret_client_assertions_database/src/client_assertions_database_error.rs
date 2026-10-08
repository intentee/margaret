use thiserror::Error;

use margaret_database::database_error::DatabaseError;

#[derive(Debug, Error)]
pub enum ClientAssertionsDatabaseError {
    #[error("a client assertion cannot be remembered in the database: {0}")]
    Remember(#[source] tokio_postgres::Error),

    #[error("the database remembering client assertions is unavailable: {0}")]
    Unavailable(#[source] DatabaseError),
}
