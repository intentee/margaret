use std::sync::Arc;

use rustls::ClientConfig;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_websocket_client::web_socket_client::WebSocketClient;
use margaret_websocket_session::web_socket_session_factory::WebSocketSessionFactory;

use crate::running_web_socket_server::RunningWebSocketServer;
use crate::test_session::TestSession;

pub struct MtlsWebSocketEndpoint {
    pub client: WebSocketClient,
    pub server: RunningWebSocketServer,
    pub url: Url,
}

impl MtlsWebSocketEndpoint {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start<Factory>(factory: Factory) -> Self
    where
        Factory: WebSocketSessionFactory<Session = TestSession> + 'static,
    {
        let fixture = MtlsFixture::new();
        let server =
            RunningWebSocketServer::start_mutually_authenticated(factory, fixture.server_config)
                .await;
        let url = Url::parse(&format!(
            "wss://{}:{}/ws",
            fixture.server_name,
            server.address().port()
        ))
        .expect("the endpoint url parses");

        Self {
            client: WebSocketClient::new(ClientConfig::clone(&fixture.client_config)),
            server,
            url,
        }
    }
}

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
#[must_use]
pub fn unreachable_client() -> WebSocketClient {
    WebSocketClient::new(ClientConfig::clone(&Arc::clone(
        &MtlsFixture::new().client_config,
    )))
}
