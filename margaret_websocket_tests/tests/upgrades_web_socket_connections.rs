use futures_util::SinkExt;
use futures_util::StreamExt;
use margaret_websocket_tests::failing_session_factory::FailingSessionFactory;
use margaret_websocket_tests::raw_http_exchange::raw_http_exchange;
use margaret_websocket_tests::redirecting_session_factory::RedirectingSessionFactory;
use margaret_websocket_tests::rejecting_session_factory::RejectingSessionFactory;
use margaret_websocket_tests::running_web_socket_server::RunningWebSocketServer;
use margaret_websocket_tests::valid_raw_handshake::valid_raw_handshake;
use margaret_websocket_tests::validating_session_factory::ValidatingSessionFactory;
use margaret_websocket_tests::web_socket_client_request::web_socket_client_request;
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
    let (mut socket, _response) = client_async(web_socket_client_request(url), tcp)
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

    let response = raw_http_exchange(server.address(), &valid_raw_handshake("/ws")).await;

    assert!(response.contains(" 403 "));

    server.stop().await;
}

#[tokio::test]
async fn reports_a_session_factory_failure_as_a_generic_server_error() {
    let server = RunningWebSocketServer::start_with(FailingSessionFactory).await;

    let response = raw_http_exchange(server.address(), &valid_raw_handshake("/ws")).await;

    assert!(response.contains(" 500 "));
    assert!(response.contains("Internal Server Error"));
    assert!(!response.contains("secret database endpoint"));

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

    let (socket, _response) = client_async(web_socket_client_request(url), tcp)
        .await
        .expect("the websocket handshake succeeds once the query form validates");

    drop(socket);
    server.stop().await;
}

#[tokio::test]
async fn rejects_the_upgrade_when_the_query_form_is_invalid() {
    let server = RunningWebSocketServer::start_with(ValidatingSessionFactory).await;

    let response = raw_http_exchange(server.address(), &valid_raw_handshake("/ws?label=")).await;

    assert!(response.contains(" 422 "));
    assert!(!response.contains(" 101 "));

    server.stop().await;
}

#[tokio::test]
async fn redirects_instead_of_upgrading_when_the_session_factory_interrupts() {
    let server = RunningWebSocketServer::start_with(RedirectingSessionFactory::new(
        margaret_http::route_origin::RouteOrigin::parse("https://example.test")
            .expect("the redirect origin is a canonical HTTPS origin"),
    ))
    .await;

    let response = raw_http_exchange(server.address(), &valid_raw_handshake("/ws")).await;

    assert!(response.contains(" 303 "));
    assert!(response.contains("location: https://example.test/sign-in"));
    assert!(!response.contains(" 101 "));

    server.stop().await;
}
