use crate::server_transport_policy::ServerTransportPolicy;
use crate::web_socket_session_route::WebSocketSessionRoute;

#[derive(Debug)]
pub struct WebSocketServerRequirements {
    pub sessions: Vec<WebSocketSessionRoute>,
    pub transport_policy: ServerTransportPolicy,
}
