use oauth2::AccessToken;
use oauth2::basic::BasicErrorResponse;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;

use crate::issued_token_type_mismatch::IssuedTokenTypeMismatch;

pub enum ExchangedToken {
    Exchanged(AccessToken),
    IssuedTokenTypeMismatch(IssuedTokenTypeMismatch),
    Refused(BasicErrorResponse),
    Unavailable(ServerUnavailability),
}
