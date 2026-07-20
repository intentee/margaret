use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use margaret_websocket_tests::running_echo_server::RunningEchoServer;

#[tokio::test]
async fn rejects_a_plain_get_without_a_websocket_handshake() {
    let server = RunningEchoServer::start().await;
    let address = server.address();

    let mut stream = TcpStream::connect(address)
        .await
        .expect("the client connects to the server");

    stream
        .write_all(b"GET /ws HTTP/1.1\r\nHost: test\r\n\r\n")
        .await
        .expect("the plain request reaches the server");

    let mut head = Vec::new();
    let mut byte = [0u8; 1];

    while !head.ends_with(b"\r\n\r\n") {
        let read = stream
            .read(&mut byte)
            .await
            .expect("the response head is readable");

        assert_ne!(read, 0, "the server closed before finishing the response");

        head.push(byte[0]);
    }

    let head = String::from_utf8_lossy(&head);

    assert!(
        head.contains("426"),
        "a plain GET is answered with 426 Upgrade Required, got: {head}"
    );

    server.stop().await;
}
