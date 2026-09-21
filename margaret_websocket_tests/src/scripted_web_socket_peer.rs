use std::net::SocketAddr;

use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::request_id::RequestId;

use crate::scripted_peer_closing::ScriptedPeerClosing;
use crate::scripted_peer_step::ScriptedPeerStep;
use crate::scripted_web_socket_peer_params::ScriptedWebSocketPeerParams;

async fn await_client_frame<Io>(web_socket: &mut WebSocketStream<Io>) -> Message
where
    Io: AsyncRead + AsyncWrite + Unpin,
{
    web_socket
        .next()
        .await
        .expect("the client sends a frame")
        .expect("the client frame reads cleanly")
}

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
                    ScriptedPeerStep::AwaitClientCancel => {
                        let frame = await_client_frame(&mut web_socket).await;

                        assert!(
                            matches!(
                                serde_json::from_str::<ClientSentFrame>(
                                    frame.to_text().expect("the client frame carries text")
                                ),
                                Ok(ClientSentFrame::Cancel { ref id })
                                    if *id == RequestId::Number(0)
                            ),
                            "an abandoned exchange is cancelled on the wire"
                        );
                    }
                    ScriptedPeerStep::AwaitClientCredit => {
                        let frame = await_client_frame(&mut web_socket).await;

                        assert!(
                            matches!(
                                serde_json::from_str::<ClientSentFrame>(
                                    frame.to_text().expect("the client frame carries text")
                                ),
                                Ok(ClientSentFrame::Credit { credit, .. }) if credit.frames() > 0
                            ),
                            "the client replenishes the window as its consumer drains"
                        );
                    }
                    ScriptedPeerStep::AwaitClientFrame => {
                        await_client_frame(&mut web_socket).await;
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
