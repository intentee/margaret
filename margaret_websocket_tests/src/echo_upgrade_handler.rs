use async_trait::async_trait;

use margaret_http::handler::Handler;
use margaret_http::request::Request;
use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_websocket::websocket_accept_key::websocket_accept_key;
use margaret_websocket::websocket_channel_config::WebSocketChannelConfig;
use margaret_websocket::websocket_upgrade::WebsocketUpgrade;

use crate::echo_protocol::EchoProtocol;

pub struct EchoUpgradeHandler;

#[async_trait]
impl Handler for EchoUpgradeHandler {
    async fn handle(&self, request: &Request) -> ResponseContinuation {
        let server = &request.inputs.server;

        match websocket_accept_key(
            server.header("upgrade").ok().flatten(),
            server.header("connection").ok().flatten(),
            server.header("sec-websocket-version").ok().flatten(),
            server.header("sec-websocket-key").ok().flatten(),
        ) {
            Some(accept_key) => ResponseContinuation::Upgrade(Box::new(WebsocketUpgrade::new(
                EchoProtocol,
                accept_key,
                WebSocketChannelConfig::recommended(),
            ))),
            None => {
                ResponseContinuation::Done(Response::text(426, "Upgrade Required"))
            }
        }
    }
}
