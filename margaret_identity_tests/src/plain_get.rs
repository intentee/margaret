use std::net::SocketAddr;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn plain_get(address: SocketAddr, path: &str) -> String {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the server");

    stream
        .write_all(
            format!("GET {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
                .as_bytes(),
        )
        .await
        .expect("the request is written");

    let mut response = Vec::new();

    stream
        .read_to_end(&mut response)
        .await
        .expect("the response is read to completion");

    String::from_utf8_lossy(&response).into_owned()
}
