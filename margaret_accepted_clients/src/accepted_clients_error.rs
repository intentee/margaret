use thiserror::Error;

#[derive(Debug, Error)]
pub enum AcceptedClientsError {
    #[error("the application could not remember a client assertion of '{client_id}': {source}")]
    RememberClientAssertion {
        client_id: &'static str,
        #[source]
        source: anyhow::Error,
    },
}
