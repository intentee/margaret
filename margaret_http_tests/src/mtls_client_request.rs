use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ClientConfig;
use rustls::pki_types::ServerName;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub async fn mtls_client_request(
    client_config: Arc<ClientConfig>,
    server_name: &str,
    address: SocketAddr,
) -> String {
    let connector = TlsConnector::from(client_config);
    let stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the mTLS server");
    let domain = ServerName::try_from(server_name.to_string()).expect("the server name is valid");
    let mut tls = connector
        .connect(domain, stream)
        .await
        .expect("the mTLS handshake succeeds");

    tls.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("the request is written");

    let mut response = Vec::new();

    tls.read_to_end(&mut response)
        .await
        .expect("the response is read to completion");

    String::from_utf8_lossy(&response).into_owned()
}
