use std::sync::Arc;

use oauth2::AccessToken;
use oauth2::basic::BasicErrorResponse;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;

#[derive(Clone)]
pub enum AcquiredToken {
    Acquired(AccessToken),
    Refused(BasicErrorResponse),
    Unavailable(Arc<ServerUnavailability>),
}
