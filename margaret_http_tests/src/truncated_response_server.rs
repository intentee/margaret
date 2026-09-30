use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ServerConfig;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;

const HEADER_TERMINATOR: &[u8] = b"\r\n\r\n";
const SENT_BODY: &str = "truncated";

fn truncated_response() -> String {
    let declared_length = SENT_BODY.len() + 1;

    format!("HTTP/1.1 200 OK\r\ncontent-length: {declared_length}\r\n\r\n{SENT_BODY}")
}

pub struct TruncatedResponseServer {
    address: SocketAddr,
    join_handle: JoinHandle<()>,
}

impl TruncatedResponseServer {
    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn start(server_config: Arc<ServerConfig>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("the truncating server binds");
        let address = listener
            .local_addr()
            .expect("the truncating listener reports its address");
        let acceptor = TlsAcceptor::from(server_config);
        let join_handle = tokio::spawn(async move {
            let (stream, _peer) = listener.accept().await.expect("the client connects");
            let mut tls = acceptor
                .accept(stream)
                .await
                .expect("the TLS handshake succeeds");
            let mut request = Vec::new();

            while !request
                .windows(HEADER_TERMINATOR.len())
                .any(|window| window == HEADER_TERMINATOR)
            {
                let read = tls
                    .read_buf(&mut request)
                    .await
                    .expect("the request is read");

                assert_ne!(read, 0, "the client closed before sending its request");
            }

            tls.write_all(truncated_response().as_bytes())
                .await
                .expect("the truncated response is written");
            tls.shutdown().await.expect("the connection closes");
        });

        Self {
            address,
            join_handle,
        }
    }

    /// # Panics
    ///
    /// Panics when the fixture it builds cannot be prepared.
    pub async fn finish(self) {
        self.join_handle
            .await
            .expect("the truncating server task finishes cleanly");
    }

    #[must_use]
    pub fn port(&self) -> u16 {
        self.address.port()
    }
}
