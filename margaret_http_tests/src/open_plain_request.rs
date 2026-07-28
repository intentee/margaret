use std::net::SocketAddr;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn open_plain_request(address: SocketAddr) -> TcpStream {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the plaintext server");

    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .await
        .expect("the request is written");

    stream
}
