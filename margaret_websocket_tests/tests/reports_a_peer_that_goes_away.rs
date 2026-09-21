use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_client::response_item::ResponseItem;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;
use margaret_websocket_tests::scripted_response_chunk::scripted_response_chunk;
use margaret_websocket_tests::typing_notification::TypingNotification;

async fn abandoned_exchange(
    closing: ScriptedPeerClosing,
    script: Vec<ScriptedPeerStep>,
) -> ScriptedPeerEndpoint {
    let endpoint = ScriptedPeerEndpoint::start(closing, script).await;

    endpoint
        .connection
        .notify(TypingNotification {
            who: "alice".to_string(),
        })
        .await
        .expect("the notification reaches a peer that is still connected");

    endpoint
}

#[tokio::test]
async fn reports_a_clean_close_that_arrives_mid_exchange() {
    let endpoint = abandoned_exchange(
        ScriptedPeerClosing::Cleanly,
        vec![ScriptedPeerStep::AwaitClientFrame],
    )
    .await;
    let mut responses = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "unanswered".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    endpoint.peer.finish().await;

    assert!(matches!(
        responses
            .next()
            .await
            .expect("an abandoned exchange reports why it ended")
            .expect_err("the peer closed before answering"),
        WebSocketClientError::ExchangePeerClosed { .. }
    ));
    assert!(responses.next().await.is_none());
}

#[tokio::test]
async fn reports_a_truncated_stream_instead_of_a_clean_finish() {
    let endpoint = ScriptedPeerEndpoint::start(
        ScriptedPeerClosing::Abruptly,
        vec![
            ScriptedPeerStep::AwaitClientFrame,
            ScriptedPeerStep::Send(Message::binary(vec![0_u8])),
            scripted_response_chunk(RequestId::Number(0), false, "partial"),
        ],
    )
    .await;
    let mut responses = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "truncated".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    assert!(matches!(
        responses
            .next()
            .await
            .expect("the peer streams one chunk")
            .expect("the chunk reads as a response chunk"),
        ResponseItem::Payload(_)
    ));

    endpoint.peer.finish().await;

    assert!(matches!(
        responses
            .next()
            .await
            .expect("a truncated stream reports why it ended")
            .expect_err("the connection died before the stream completed"),
        WebSocketClientError::ExchangeConnectionFailed { .. }
    ));
}

#[tokio::test]
async fn reports_a_notification_the_closed_connection_cannot_carry() {
    let endpoint = abandoned_exchange(
        ScriptedPeerClosing::Cleanly,
        vec![ScriptedPeerStep::AwaitClientFrame],
    )
    .await;

    endpoint.peer.finish().await;

    assert!(matches!(
        endpoint
            .connection
            .notify(TypingNotification {
                who: "bob".to_string(),
            })
            .await
            .expect_err("a closed connection carries nothing"),
        WebSocketClientError::SendFrame { .. }
    ));
}

#[tokio::test]
async fn reports_a_request_the_closed_connection_cannot_carry() {
    let endpoint = abandoned_exchange(
        ScriptedPeerClosing::Cleanly,
        vec![ScriptedPeerStep::AwaitClientFrame],
    )
    .await;

    endpoint.peer.finish().await;

    let refused = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "refused".to_string(),
        })
        .await
        .map(drop)
        .expect_err("a closed connection carries no request");

    assert!(matches!(refused, WebSocketClientError::SendFrame { .. }));
}
