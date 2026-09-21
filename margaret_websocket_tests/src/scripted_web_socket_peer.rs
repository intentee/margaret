use std::net::SocketAddr;

use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;

use crate::scripted_peer_closing::ScriptedPeerClosing;
use crate::scripted_peer_step::ScriptedPeerStep;
use crate::scripted_web_socket_peer_params::ScriptedWebSocketPeerParams;

pub struct ScriptedWebSocketPeer {
    address: SocketAddr,
    join_handle: JoinHandle<()>,
}

impl ScriptedWebSocketPeer {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start(
        ScriptedWebSocketPeerParams {
            closing,
            script,
            server_config,
        }: ScriptedWebSocketPeerParams,
    ) -> Self {
        let listener = TcpListener::bind("localhost:0")
            .await
            .expect("the scripted peer binds");
        let address = listener
            .local_addr()
            .expect("the bound listener reports its address");
        let acceptor = TlsAcceptor::from(server_config);
        let join_handle = tokio::spawn(async move {
            let (stream, _remote) = listener.accept().await.expect("a client connects");

            stream
                .set_nodelay(true)
                .expect("the scripted peer answers without waiting on Nagle");
            let tls_stream = acceptor
                .accept(stream)
                .await
                .expect("the client completes the tls handshake");
            let mut web_socket = tokio_tungstenite::accept_async(tls_stream)
                .await
                .expect("the client completes the websocket handshake");

            for step in script {
                match step {
                    ScriptedPeerStep::AwaitClientFrame => {
                        web_socket
                            .next()
                            .await
                            .expect("the client sends a frame")
                            .expect("the client frame reads cleanly");
                    }
                    ScriptedPeerStep::Send(message) => {
                        web_socket
                            .send(message)
                            .await
                            .expect("the scripted message is sent");
                    }
                }
            }

            match closing {
                ScriptedPeerClosing::Abruptly => drop(web_socket),
                ScriptedPeerClosing::StaysOpen => while web_socket.next().await.is_some() {},
                ScriptedPeerClosing::Cleanly => {
                    web_socket
                        .close(None)
                        .await
                        .expect("the scripted peer closes cleanly");

                    while web_socket.next().await.is_some() {}

                    web_socket
                        .get_mut()
                        .shutdown()
                        .await
                        .expect("the scripted peer shuts the tls session down cleanly");
                }
            }
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
