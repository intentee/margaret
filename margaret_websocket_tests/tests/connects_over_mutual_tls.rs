use std::sync::Arc;

use url::Url;

use margaret_websocket_client::response_item::ResponseItem;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_tests::conversation_message::ConversationMessage;
use margaret_websocket_tests::mistyped_chunk::MistypedChunk;
use margaret_websocket_tests::mtls_web_socket_endpoint::MtlsWebSocketEndpoint;
use margaret_websocket_tests::mtls_web_socket_endpoint::unreachable_client;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::running_web_socket_server::RunningWebSocketServer;
use margaret_websocket_tests::shared_session_factory::SharedSessionFactory;
use margaret_websocket_tests::test_session::TestSession;
use margaret_websocket_tests::test_session_factory::TestSessionFactory;
use margaret_websocket_tests::typing_notification::TypingNotification;

fn url(candidate: &str) -> Url {
    Url::parse(candidate).expect("the fixture url parses")
}

#[tokio::test]
async fn answers_a_request_over_mutual_tls() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");
    let mut responses = connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "hello".to_string(),
        })
        .await
        .expect("the request is sent");

    let first = responses
        .next()
        .await
        .expect("the peer answers")
        .expect("the answer reads as a response chunk");

    assert!(matches!(first, ResponseItem::Payload(chunk) if chunk.text == "pong hello"));
    assert!(responses.next().await.is_none());

    drop(connection);
    endpoint.server.stop().await;
}

#[tokio::test]
async fn surfaces_a_peer_rejection_as_an_outcome() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");
    let mut responses = connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: String::new(),
        })
        .await
        .expect("the request is sent");

    let rejection = responses
        .next()
        .await
        .expect("the peer answers")
        .expect("a rejection is an outcome, not a failure");

    assert!(matches!(rejection, ResponseItem::Rejected(_)));

    drop(connection);
    endpoint.server.stop().await;
}

#[tokio::test]
async fn reports_a_response_method_the_request_did_not_ask_for() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");
    let mut responses = connection
        .request::<ConversationMessage, ResponseChunk>(ConversationMessage {
            prompt: "cats".to_string(),
        })
        .await
        .expect("the request is sent");

    assert!(matches!(
        responses
            .next()
            .await
            .expect("the peer streams a chunk")
            .expect("the chunk reads as a response chunk"),
        ResponseItem::Payload(_)
    ));
    assert!(matches!(
        responses
            .next()
            .await
            .expect("the peer closes the stream")
            .expect_err("the closing frame carries another response method"),
        WebSocketClientError::UnexpectedResponseMethod {
            expected: "response_chunk",
            ..
        }
    ));

    drop(connection);
    endpoint.server.stop().await;
}

#[tokio::test]
async fn delivers_a_notification_to_the_peer() {
    let session = Arc::new(TestSession::default());
    let endpoint = MtlsWebSocketEndpoint::start(SharedSessionFactory::new(session.clone())).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");

    connection
        .notify(TypingNotification {
            who: "alice".to_string(),
        })
        .await
        .expect("the notification is sent");

    let mut responses = connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "after".to_string(),
        })
        .await
        .expect("the request is sent");

    assert!(responses.next().await.is_some());
    assert_eq!(
        session
            .notifications
            .lock()
            .expect("the notification log is not poisoned")
            .as_slice(),
        ["alice".to_string()]
    );

    drop(responses);
    drop(connection);
    endpoint.server.stop().await;
}

async fn connection_failure(candidate: &str) -> WebSocketClientError {
    match unreachable_client().connect(&url(candidate)).await {
        Ok(_) => panic!("'{candidate}' must not yield a connection"),
        Err(error) => error,
    }
}

#[tokio::test]
async fn refuses_a_url_that_is_not_a_secure_websocket_endpoint() {
    assert!(matches!(
        connection_failure("ws://localhost:9/ws").await,
        WebSocketClientError::UnusableUrl { .. }
    ));
}

#[tokio::test]
async fn refuses_a_url_whose_host_is_not_a_tls_server_name() {
    assert!(matches!(
        connection_failure("wss://[::1]:9/ws").await,
        WebSocketClientError::TlsServerName { .. }
    ));
}

#[tokio::test]
async fn reports_a_refused_tcp_connection() {
    assert!(matches!(
        connection_failure("wss://localhost:9/ws").await,
        WebSocketClientError::TcpConnect { .. }
    ));
}

#[tokio::test]
async fn reports_a_response_payload_that_does_not_match_the_requested_type() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");
    let mut responses = connection
        .request::<PingMessage, MistypedChunk>(PingMessage {
            label: "hello".to_string(),
        })
        .await
        .expect("the request is sent");

    assert!(matches!(
        responses
            .next()
            .await
            .expect("the peer answers")
            .expect_err("the payload does not fit the requested type"),
        WebSocketClientError::DeserializeResponse { .. }
    ));

    drop(responses);
    drop(connection);
    endpoint.server.stop().await;
}

#[tokio::test]
async fn reports_a_tls_handshake_against_a_plaintext_peer() {
    let server = RunningWebSocketServer::start().await;
    let candidate = format!("wss://127.0.0.1:{}/ws", server.address().port());

    assert!(matches!(
        match unreachable_client().connect(&url(&candidate)).await {
            Ok(_) => panic!("a plaintext peer must not complete a tls handshake"),
            Err(error) => error,
        },
        WebSocketClientError::TlsHandshake { .. }
    ));

    server.stop().await;
}

#[tokio::test]
async fn reports_a_websocket_handshake_the_peer_refuses() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let refused = url(&format!(
        "wss://localhost:{}/absent",
        endpoint.server.address().port()
    ));

    assert!(matches!(
        match endpoint.client.connect(&refused).await {
            Ok(_) => panic!("the peer serves no websocket route at '/absent'"),
            Err(error) => error,
        },
        WebSocketClientError::WebSocketHandshake { .. }
    ));

    endpoint.server.stop().await;
}

#[tokio::test]
async fn ends_the_response_stream_when_the_peer_goes_away() {
    let endpoint = MtlsWebSocketEndpoint::start(TestSessionFactory).await;
    let connection = endpoint
        .client
        .connect(&endpoint.url)
        .await
        .expect("the client connects over mutual tls");
    let mut responses = connection
        .request::<ConversationMessage, ResponseChunk>(ConversationMessage {
            prompt: "cats".to_string(),
        })
        .await
        .expect("the request is sent");

    assert!(responses.next().await.is_some());

    endpoint.server.stop().await;

    while responses.next().await.is_some() {}

    drop(connection);
}
