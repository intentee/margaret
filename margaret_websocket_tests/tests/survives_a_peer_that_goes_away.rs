use rustls::ClientConfig;
use tokio_tungstenite::tungstenite::Message;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_websocket_client::response_stream::ResponseStream;
use margaret_websocket_client::web_socket_client::WebSocketClient;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_client::web_socket_connection::WebSocketConnection;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_web_socket_peer::ScriptedWebSocketPeer;
use margaret_websocket_tests::typing_notification::TypingNotification;

struct AbandonedExchange {
    connection: WebSocketConnection,
    responses: ResponseStream<ResponseChunk>,
}

async fn abandoned_exchange(script: Vec<Message>) -> AbandonedExchange {
    let fixture = MtlsFixture::new();
    let peer = ScriptedWebSocketPeer::start(fixture.server_config, script).await;
    let url = Url::parse(&format!(
        "wss://{}:{}/ws",
        fixture.server_name,
        peer.address().port()
    ))
    .expect("the peer url parses");
    let connection = WebSocketClient::new(ClientConfig::clone(&fixture.client_config))
        .connect(&url)
        .await
        .expect("the client connects to the scripted peer");
    let responses = connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "unanswered".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    peer.finish().await;

    AbandonedExchange {
        connection,
        responses,
    }
}

#[tokio::test]
async fn ends_an_unanswered_exchange_when_the_peer_closes() {
    let mut exchange = abandoned_exchange(vec![Message::text("this is not an envelope")]).await;

    assert!(exchange.responses.next().await.is_none());
}

#[tokio::test]
async fn reports_a_notification_the_closed_connection_cannot_carry() {
    let mut exchange = abandoned_exchange(vec![Message::binary(vec![0_u8])]).await;

    assert!(exchange.responses.next().await.is_none());
    assert!(matches!(
        exchange
            .connection
            .notify(TypingNotification {
                who: "alice".to_string(),
            })
            .await
            .expect_err("a closed connection carries nothing"),
        WebSocketClientError::SendFrame { .. }
    ));
}

#[tokio::test]
async fn reports_a_request_the_closed_connection_cannot_carry() {
    let mut exchange = abandoned_exchange(Vec::new()).await;

    assert!(exchange.responses.next().await.is_none());

    let refused = exchange
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "refused".to_string(),
        })
        .await;

    assert!(matches!(
        match refused {
            Ok(_) => panic!("a closed connection carries no request"),
            Err(error) => error,
        },
        WebSocketClientError::SendFrame { .. }
    ));
}
