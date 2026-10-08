use thiserror::Error;

use margaret_client_assertions::client_assertions_error::ClientAssertionsError;

#[derive(Debug, Error)]
pub enum AcceptedClientsError {
    #[error("the application could not remember a client assertion of '{client_id}': {source}")]
    RememberClientAssertion {
        client_id: &'static str,
        #[source]
        source: ClientAssertionsError,
    },
}
