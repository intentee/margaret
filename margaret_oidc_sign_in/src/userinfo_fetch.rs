use http::StatusCode;

use margaret_authorization_server_client::server_unavailability::ServerUnavailability;

pub enum UserinfoFetch<TUserinfo> {
    Fetched(TUserinfo),
    Refused { status: StatusCode },
    SubjectMismatch { found: String },
    Unavailable(ServerUnavailability),
}
