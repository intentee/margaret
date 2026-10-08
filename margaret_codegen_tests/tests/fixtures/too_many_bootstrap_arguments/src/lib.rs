use margaret::framework::websocket::web_socket_response::WebSocketResponse;
use margaret::framework::route_method::route_method::RouteMethod;
use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[renders_view(name = "configured")]
#[responds_to_http(method = RouteMethod::Get, path = "/configured", server = "public")]
#[singleton]
struct Configured;

impl Configured {
    #[constructor]
    fn create(
        #[console_argument(from = "alpha")] alpha: String,
        #[console_argument(from = "bravo")] bravo: String,
        #[console_argument(from = "charlie")] charlie: String,
        #[console_argument(from = "delta")] delta: String,
        #[console_argument(from = "echo")] echo: String,
        #[console_argument(from = "foxtrot")] foxtrot: String,
        #[console_argument(from = "golf")] golf: String,
    ) -> anyhow::Result<Self> {
    }

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
