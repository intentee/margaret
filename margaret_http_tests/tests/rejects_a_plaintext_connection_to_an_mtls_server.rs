use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_mtls_server::RunningMtlsServer;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::test]
async fn rejects_a_plaintext_connection_to_an_mtls_server() {
    let fixture = MtlsFixture::new();
    let server = RunningMtlsServer::start(fixture.server_config.clone()).await;

    let mut stream = TcpStream::connect(server.address())
        .await
        .expect("the plaintext client connects to the mTLS server");
    let _write_outcome = stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .expect("the connection is read to completion");

    assert!(!String::from_utf8_lossy(&response).contains(" 200 "));

    server.stop().await;
}
