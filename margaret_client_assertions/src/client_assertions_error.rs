use thiserror::Error;

use margaret::framework::active_record::active_record_error::ActiveRecordError;

#[derive(Debug, Error)]
pub enum ClientAssertionsError {
    #[error("a client assertion cannot be remembered in the database: {0}")]
    Remember(#[source] ActiveRecordError),

    #[error("the expired client assertions cannot be swept from the database: {0}")]
    Sweep(#[source] ActiveRecordError),
}
