use futures_util::SinkExt;
use futures_util::StreamExt;
use margaret_websocket_tests::raw_http_exchange::raw_http_exchange;
use margaret_websocket_tests::rejecting_session_factory::RejectingSessionFactory;
use margaret_websocket_tests::running_websocket_server::RunningWebSocketServer;
use margaret_websocket_tests::validating_session_factory::ValidatingSessionFactory;
use tokio::net::TcpStream;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn upgrades_a_connection_and_serves_a_request() {
    let server = RunningWebSocketServer::start().await;
    let url = format!("ws://{}/ws", server.address());
    let tcp = TcpStream::connect(server.address())
        .await
        .expect("the client connects to the server");
    let (mut socket, _response) = client_async(url, tcp)
        .await
        .expect("the websocket handshake succeeds");

    socket
        .send(Message::text(
            r#"{"id":1,"method":"ping","params":{"label":"here"}}"#.to_owned(),
        ))
        .await
        .expect("the client sends a request");

    let response = socket
        .next()
        .await
        .expect("a websocket frame arrives")
        .expect("the websocket frame reads cleanly")
        .to_text()
        .expect("the frame is text")
        .to_owned();

    assert!(response.contains("pong here"));

    drop(socket);
    server.stop().await;
}

#[tokio::test]
async fn rejects_a_handshake_missing_the_key() {
    let server = RunningWebSocketServer::start().await;

    let response = raw_http_exchange(
        server.address(),
        b"GET /ws HTTP/1.1\r\nHost: test\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.contains(" 400 "));

    server.stop().await;
}

#[tokio::test]
async fn rejects_the_upgrade_when_the_session_factory_rejects() {
    let server = RunningWebSocketServer::start_with(RejectingSessionFactory).await;

    let response = raw_http_exchange(
        server.address(),
        b"GET /ws HTTP/1.1\r\nHost: test\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.contains(" 403 "));

    server.stop().await;
}

#[tokio::test]
async fn rejects_a_handshake_with_an_unsupported_version() {
    let server = RunningWebSocketServer::start().await;

    let response = raw_http_exchange(
        server.address(),
        b"GET /ws HTTP/1.1\r\nHost: test\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 8\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.contains(" 400 "));

    server.stop().await;
}

#[tokio::test]
async fn upgrades_when_the_query_form_validates() {
    let server = RunningWebSocketServer::start_with(ValidatingSessionFactory).await;
    let url = format!("ws://{}/ws?label=here", server.address());
    let tcp = TcpStream::connect(server.address())
        .await
        .expect("the client connects to the server");

    let (socket, _response) = client_async(url, tcp)
        .await
        .expect("the websocket handshake succeeds once the query form validates");

    drop(socket);
    server.stop().await;
}

#[tokio::test]
async fn rejects_the_upgrade_when_the_query_form_is_invalid() {
    let server = RunningWebSocketServer::start_with(ValidatingSessionFactory).await;

    let response = raw_http_exchange(
        server.address(),
        b"GET /ws?label= HTTP/1.1\r\nHost: test\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.contains(" 422 "));
    assert!(!response.contains(" 101 "));

    server.stop().await;
}
