use rustls::ClientConfig;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_websocket_client::web_socket_client::WebSocketClient;
use margaret_websocket_client::web_socket_connection::WebSocketConnection;

use crate::scripted_peer_closing::ScriptedPeerClosing;
use crate::scripted_peer_step::ScriptedPeerStep;
use crate::scripted_web_socket_peer::ScriptedWebSocketPeer;
use crate::scripted_web_socket_peer_params::ScriptedWebSocketPeerParams;

pub struct ScriptedPeerEndpoint {
    pub connection: WebSocketConnection,
    pub peer: ScriptedWebSocketPeer,
}

impl ScriptedPeerEndpoint {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start(closing: ScriptedPeerClosing, script: Vec<ScriptedPeerStep>) -> Self {
        let fixture = MtlsFixture::new();
        let peer = ScriptedWebSocketPeer::start(ScriptedWebSocketPeerParams {
            closing,
            script,
            server_config: fixture.server_config,
        })
        .await;
        let url = Url::parse(&format!(
            "wss://{}:{}/ws",
            fixture.server_name,
            peer.address().port()
        ))
        .expect("the peer url parses");
        let connection = WebSocketClient::new(ClientConfig::clone(&fixture.client_config))
            .connect(&url)
            .await
            .expect("the client connects to the scripted peer");

        Self { connection, peer }
    }
}
