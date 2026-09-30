use crate::accepted_client::AcceptedClient;
use crate::client_refusal::ClientRefusal;

pub enum ClientAuthenticationOutcome<'clients> {
    Authenticated(&'clients AcceptedClient),
    Refused(ClientRefusal),
}
