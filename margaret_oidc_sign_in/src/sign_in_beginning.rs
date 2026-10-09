use margaret_authorization_server_client::server_unavailability::ServerUnavailability;
use margaret_http::response::Response;

pub enum SignInBeginning {
    Redirected(Response),
    Unavailable(ServerUnavailability),
}
