use thiserror::Error;

use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_registered_claims::audience::Audience;

#[derive(Debug, Error)]
pub enum AcceptedClientsError {
    #[error("the accepted client '{client_id}' is declared more than once")]
    DuplicateClientId { client_id: ClientId },

    #[error(
        "the accepted client '{client_id}' names the session audience '{audience}' as one of its resources"
    )]
    SessionAudienceResource {
        audience: Audience,
        client_id: ClientId,
    },

    #[error(
        "the accepted client '{client_id}' is a uuid, which could be mistaken for the subject of a user"
    )]
    UuidClientId { client_id: ClientId },
}
