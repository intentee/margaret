use margaret::framework::websocket::responds_to_web_socket_message::RespondsToWebSocketMessage;

#[renders_view(name = "configured")]
#[responds_to_http(access = margaret::framework::http::public_access::PublicAccess, method = "get", path = "/configured", server = "public")]
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

#[websocket_session(access = margaret::framework::http::public_access::PublicAccess, origin = "https://example.test", path = "/room", server = "public")]
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
