use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ClientConfig;
use rustls::pki_types::ServerName;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub async fn mtls_connect(
    client_config: Arc<ClientConfig>,
    server_name: &str,
    address: SocketAddr,
) -> TlsStream<TcpStream> {
    let connector = TlsConnector::from(client_config);
    let stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the mTLS server");
    let domain = ServerName::try_from(server_name.to_string()).expect("the server name is valid");

    connector
        .connect(domain, stream)
        .await
        .expect("the mTLS handshake succeeds")
}
