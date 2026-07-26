use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[renders_view(name = "configured")]
#[responds_to_http(method = "get", path = "/configured", server = "public")]
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

#[websocket_message(request, method = "ping", response = single)]
struct Ping;

impl RespondsToWebSocketMessage for Configured {
    type Session = Room;
    type Message = Ping;
}
