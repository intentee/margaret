use spiffe::spiffe_id::SpiffeId;

use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[websocket_session(path = "/gateway", server = "gateway")]
struct GatewaySession;

impl GatewaySession {
    #[build_for_session]
    fn build_for_session(peer: &SpiffeId) -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = "ping", response = single)]
struct Ping;

#[singleton]
struct Ponger;

impl Ponger {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}

impl RespondsToWebSocketMessage for Ponger {
    type Session = GatewaySession;
    type Message = Ping;
}
