use std::net::SocketAddr;
use std::sync::Arc;

use futures_util::SinkExt;
use futures_util::StreamExt;
use rustls::ServerConfig;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::tungstenite::Message;

pub struct ScriptedWebSocketPeer {
    address: SocketAddr,
    join_handle: JoinHandle<()>,
}

impl ScriptedWebSocketPeer {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start(server_config: Arc<ServerConfig>, script: Vec<Message>) -> Self {
        let listener = TcpListener::bind("localhost:0")
            .await
            .expect("the scripted peer binds");
        let address = listener
            .local_addr()
            .expect("the bound listener reports its address");
        let acceptor = TlsAcceptor::from(server_config);
        let join_handle = tokio::spawn(async move {
            let (stream, _remote) = listener.accept().await.expect("a client connects");
            let tls_stream = acceptor
                .accept(stream)
                .await
                .expect("the client completes the tls handshake");
            let mut web_socket = tokio_tungstenite::accept_async(tls_stream)
                .await
                .expect("the client completes the websocket handshake");

            for message in script {
                web_socket
                    .send(message)
                    .await
                    .expect("the scripted message is sent");
            }

            web_socket
                .close(None)
                .await
                .expect("the scripted peer closes cleanly");

            while web_socket.next().await.is_some() {}
        });

        Self {
            address,
            join_handle,
        }
    }

    #[must_use]
    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// # Panics
    ///
    /// Panics when the scripted peer does not finish its script.
    pub async fn finish(self) {
        self.join_handle
            .await
            .expect("the scripted peer finishes its script");
    }
}
