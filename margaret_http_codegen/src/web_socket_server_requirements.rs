use margaret_serve_input_codegen::serve_input::ServeInput;

use crate::server_transport_policy::ServerTransportPolicy;

#[derive(Debug)]
pub struct WebSocketServerRequirements {
    pub serve_inputs: Vec<ServeInput>,
    pub transport_policy: ServerTransportPolicy,
}
