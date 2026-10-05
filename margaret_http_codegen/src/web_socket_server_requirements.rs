use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::server_transport_policy::ServerTransportPolicy;
use crate::web_socket_session_route::WebSocketSessionRoute;

#[derive(Debug)]
pub struct WebSocketServerRequirements {
    pub serve_inputs: Vec<ServeInput>,
    pub sessions: Vec<WebSocketSessionRoute>,
    pub transport_policy: ServerTransportPolicy,
}
