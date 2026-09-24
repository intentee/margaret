use std::net::SocketAddr;
use std::sync::Arc;

use rustls::ClientConfig;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

use crate::mtls_connect::mtls_connect;

/// # Panics
///
/// Panics when the fixture it builds cannot be prepared.
pub async fn mtls_client_request(
    client_config: Arc<ClientConfig>,
    server_name: &str,
    address: SocketAddr,
) -> String {
    let mut tls = mtls_connect(client_config, server_name, address).await;

    tls.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await
        .expect("the request is written");

    let mut response = Vec::new();

    tls.read_to_end(&mut response)
        .await
        .expect("the response is read to completion");

    String::from_utf8_lossy(&response).into_owned()
}
