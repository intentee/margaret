use async_trait::async_trait;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::protocol::Role;
use tokio_util::sync::CancellationToken;

use margaret_http::response::Response;
use margaret_http::upgrade_handler::UpgradeHandler;

use crate::protocol::Protocol;
use crate::run_connection::run_connection;
use crate::websocket_channel_config::WebSocketChannelConfig;

pub struct WebsocketUpgrade<TProtocol> {
    accept_key: String,
    config: WebSocketChannelConfig,
    protocol: TProtocol,
}

impl<TProtocol> WebsocketUpgrade<TProtocol> {
    #[must_use]
    pub fn new(protocol: TProtocol, accept_key: String, config: WebSocketChannelConfig) -> Self {
        Self {
            accept_key,
            config,
            protocol,
        }
    }
}

#[async_trait]
impl<TProtocol> UpgradeHandler for WebsocketUpgrade<TProtocol>
where
    TProtocol: Protocol,
{
    fn switching_response(&self) -> Response {
        Response::text(101, "")
            .header("upgrade", "websocket")
            .header("connection", "Upgrade")
            .header("sec-websocket-accept", self.accept_key.clone())
    }

    async fn serve(
        self: Box<Self>,
        upgraded: TokioIo<Upgraded>,
        connection_token: CancellationToken,
    ) {
        let stream = WebSocketStream::from_raw_socket(upgraded, Role::Server, None).await;

        if let Err(error) =
            run_connection(self.protocol, stream, connection_token, self.config).await
        {
            eprintln!("margaret_websocket: a websocket connection ended with an error: {error}");
        }
    }
}
