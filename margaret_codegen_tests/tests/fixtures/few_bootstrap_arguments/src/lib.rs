use margaret::framework::macros::build_for_session;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::renders_view;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::macros::websocket_message;
use margaret::framework::macros::websocket_session;

use margaret::framework::websocket::web_socket_response::WebSocketResponse;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[renders_view(name = "configured")]
#[responds_to_http(method = RouteMethod::Get, path = "/configured", server = "public")]
#[singleton]
struct Configured;

impl Configured {
    #[constructor]
    fn create(#[console_argument(from = "label")] label: String) -> anyhow::Result<Self> {}

    #[process]
    fn respond(&self) -> anyhow::Result<Response> {}
}

#[websocket_session(path = "/room", server = "public")]
struct Room;

impl Room {
    #[build_for_session]
    fn build() -> anyhow::Result<Self> {}
}

#[websocket_message(request, method = "ping", response = WebSocketResponse::Single)]
struct Ping;

impl RespondsToWebSocketMessage for Configured {
    type Session = Room;
    type Message = Ping;
}
