use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

/// # Panics
///
/// Panics when the fixture server cannot be reached.
pub async fn raw_exchange(address: SocketAddr, request: &[u8]) -> String {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the fixture server");

    stream
        .write_all(request)
        .await
        .expect("the request reaches the fixture server");

    let mut response = Vec::new();

    stream
        .read_to_end(&mut response)
        .await
        .expect("the response is read to completion");

    String::from_utf8_lossy(&response).into_owned()
}
