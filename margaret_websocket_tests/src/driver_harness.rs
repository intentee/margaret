use std::sync::Arc;

use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::io::DuplexStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::Role;
use tokio_util::sync::CancellationToken;

use margaret_websocket::serve_web_socket_connection::serve_web_socket_connection;
use margaret_websocket::web_socket_dispatch_table::WebSocketDispatchTable;

use crate::test_session::TestSession;

pub struct DriverHarness {
    pub cancellation_token: CancellationToken,
    pub client: WebSocketStream<DuplexStream>,
    pub driver: JoinHandle<()>,
    pub session: Arc<TestSession>,
}

impl DriverHarness {
    pub async fn spawn(dispatch_table: Arc<WebSocketDispatchTable<TestSession>>) -> Self {
        let (server_io, client_io) = tokio::io::duplex(65536);
        let server = WebSocketStream::from_raw_socket(server_io, Role::Server, None).await;
        let client = WebSocketStream::from_raw_socket(client_io, Role::Client, None).await;
        let session = Arc::new(TestSession::default());
        let cancellation_token = CancellationToken::new();
        let driver = tokio::spawn(serve_web_socket_connection(
            cancellation_token.clone(),
            session.clone(),
            dispatch_table,
            server,
        ));

        Self {
            cancellation_token,
            client,
            driver,
            session,
        }
    }

    pub async fn recv(&mut self) -> String {
        self.client
            .next()
            .await
            .expect("a websocket frame arrives")
            .expect("the websocket frame reads cleanly")
            .to_text()
            .expect("the websocket frame is text")
            .to_owned()
    }

    pub async fn send(&mut self, text: &str) {
        self.client
            .send(Message::text(text.to_owned()))
            .await
            .expect("the client sends a frame");
    }

    pub async fn send_raw(&mut self, message: Message) {
        self.client
            .send(message)
            .await
            .expect("the client sends a frame");
    }
}
