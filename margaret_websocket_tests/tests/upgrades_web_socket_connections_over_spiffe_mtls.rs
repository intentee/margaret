use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::Message;

use margaret_http::transport_config::TransportConfig;
use margaret_http_tests::mtls_connect::mtls_connect;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_websocket_tests::running_web_socket_server::RunningWebSocketServer;
use margaret_websocket_tests::verified_peer_session_factory::VerifiedPeerSessionFactory;

#[tokio::test]
async fn upgrades_a_verified_peer_and_serves_its_requests() {
    let fixture = MtlsFixture::new();
    let server = RunningWebSocketServer::start_with_transport(
        VerifiedPeerSessionFactory,
        TransportConfig::MutualTls {
            server_config: fixture.server_config.clone(),
        },
    )
    .await;
    let tls = mtls_connect(
        fixture.client_config.clone(),
        &fixture.server_name,
        server.address(),
    )
    .await;
    let (mut socket, response) = client_async("ws://localhost/ws", tls)
        .await
        .expect("the websocket handshake succeeds over SPIFFE mTLS");

    assert_eq!(response.status(), 101);

    socket
        .send(Message::text(
            r#"{"id":1,"method":"ping","params":{"label":"mesh"}}"#.to_owned(),
        ))
        .await
        .expect("the client sends a request");

    let frame = socket
        .next()
        .await
        .expect("a websocket frame arrives")
        .expect("the websocket frame reads cleanly")
        .to_text()
        .expect("the frame is text")
        .to_owned();

    assert!(frame.contains("pong mesh"));

    drop(socket);
    server.stop().await;
}

#[tokio::test]
async fn forbids_the_upgrade_of_a_peer_without_a_spiffe_id() {
    let fixture = MtlsFixture::new();
    let server = RunningWebSocketServer::start_with_transport(
        VerifiedPeerSessionFactory,
        TransportConfig::MutualTls {
            server_config: fixture.server_config.clone(),
        },
    )
    .await;
    let tls = mtls_connect(
        fixture.client_config_without_spiffe_id.clone(),
        &fixture.server_name,
        server.address(),
    )
    .await;
    let rejection = client_async("ws://localhost/ws", tls)
        .await
        .map(drop)
        .expect_err("the upgrade is refused without a peer SPIFFE id");

    assert!(matches!(rejection, Error::Http(ref response) if response.status() == 403));

    server.stop().await;
}
