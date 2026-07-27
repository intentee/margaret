use std::sync::Arc;

use futures_util::SinkExt;
use futures_util::StreamExt;
use margaret_http::http_middleware::HttpMiddleware;
use margaret_websocket_tests::blocking_middleware::BlockingMiddleware;
use margaret_websocket_tests::passing_middleware::PassingMiddleware;
use margaret_websocket_tests::raw_http_exchange::raw_http_exchange;
use margaret_websocket_tests::running_web_socket_server::RunningWebSocketServer;
use tokio::net::TcpStream;
use tokio_tungstenite::client_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn blocks_the_handshake_when_a_middleware_short_circuits() {
    let middleware: Vec<Arc<dyn HttpMiddleware>> = vec![Arc::new(BlockingMiddleware)];
    let server = RunningWebSocketServer::start_gated(middleware).await;

    let response = raw_http_exchange(
        server.address(),
        b"GET /ws HTTP/1.1\r\nHost: test\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\nSec-WebSocket-Version: 13\r\nConnection: close\r\n\r\n",
    )
    .await;

    assert!(response.contains(" 403 "));
    assert!(!response.contains(" 101 "));

    server.stop().await;
}

#[tokio::test]
async fn upgrades_the_connection_when_the_middleware_delegates() {
    let middleware: Vec<Arc<dyn HttpMiddleware>> = vec![Arc::new(PassingMiddleware)];
    let server = RunningWebSocketServer::start_gated(middleware).await;
    let url = format!("ws://{}/ws", server.address());
    let tcp = TcpStream::connect(server.address())
        .await
        .expect("the client connects to the server");
    let (mut socket, _response) = client_async(url, tcp)
        .await
        .expect("the websocket handshake succeeds once the middleware delegates");

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
