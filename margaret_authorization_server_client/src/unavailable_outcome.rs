use crate::authorization_url::AuthorizationUrl;
use crate::endpoint_outcome::EndpointOutcome;
use crate::server_unavailability::ServerUnavailability;
use crate::userinfo_outcome::UserinfoOutcome;

pub(crate) trait UnavailableOutcome {
    fn unavailable(unavailability: ServerUnavailability) -> Self;
}

impl UnavailableOutcome for AuthorizationUrl {
    fn unavailable(unavailability: ServerUnavailability) -> Self {
        Self::Unavailable(unavailability)
    }
}

impl<TAnswer> UnavailableOutcome for EndpointOutcome<TAnswer> {
    fn unavailable(unavailability: ServerUnavailability) -> Self {
        Self::Unavailable(unavailability)
    }
}

impl<TUserinfo> UnavailableOutcome for UserinfoOutcome<TUserinfo> {
    fn unavailable(unavailability: ServerUnavailability) -> Self {
        Self::Unavailable(unavailability)
    }
}
