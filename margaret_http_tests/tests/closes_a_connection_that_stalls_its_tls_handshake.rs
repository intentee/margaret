use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

use margaret_http_tests::echo_peer_route::echo_peer_route;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;

#[tokio::test(start_paused = true)]
async fn closes_a_connection_that_stalls_its_tls_handshake() {
    let fixture = MtlsFixture::new();
    let server =
        RunningFixtureServer::start(fixture.server_config.clone(), vec![echo_peer_route()]).await;
    let mut stream = TcpStream::connect(server.address())
        .await
        .expect("the silent client connects to the mTLS server");
    let mut received = Vec::new();

    stream
        .read_to_end(&mut received)
        .await
        .expect("the server closes the stalled handshake");

    assert!(received.is_empty());

    server.stop().await;
}
