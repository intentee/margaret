use url::Url;

use crate::server_unavailability::ServerUnavailability;

pub enum AuthorizationUrl {
    Built(Url),
    Unavailable(ServerUnavailability),
}
