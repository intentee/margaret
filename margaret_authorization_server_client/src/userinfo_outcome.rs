use http::StatusCode;

use crate::server_unavailability::ServerUnavailability;

pub enum UserinfoOutcome<TUserinfo> {
    Answered(TUserinfo),
    Refused { status: StatusCode },
    Unavailable(ServerUnavailability),
}
