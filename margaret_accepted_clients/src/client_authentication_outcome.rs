use crate::client_refusal::ClientRefusal;
use crate::registered_client::RegisteredClient;

pub enum ClientAuthenticationOutcome<'clients> {
    Authenticated(&'clients RegisteredClient),
    KeysAwaited,
    Refused(ClientRefusal),
}
