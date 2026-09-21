use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;
use margaret_websocket_tests::typing_notification::TypingNotification;

#[tokio::test]
async fn fails_an_outstanding_exchange_when_the_peer_sends_an_unreadable_frame() {
    let endpoint = ScriptedPeerEndpoint::start(
        ScriptedPeerClosing::Abruptly,
        vec![
            ScriptedPeerStep::AwaitClientFrame,
            ScriptedPeerStep::Send(Message::text("this is not an envelope")),
        ],
    )
    .await;
    let mut responses = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "outstanding".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    assert!(matches!(
        responses
            .next()
            .await
            .expect("a protocol violation ends the exchange instead of leaving it hanging")
            .expect_err("the peer sent a frame that is not an envelope"),
        WebSocketClientError::ExchangeProtocolViolation { .. }
    ));
}

#[tokio::test]
async fn refuses_new_work_once_the_peer_has_broken_the_protocol() {
    let endpoint = ScriptedPeerEndpoint::start(
        ScriptedPeerClosing::StaysOpen,
        vec![
            ScriptedPeerStep::AwaitClientFrame,
            ScriptedPeerStep::Send(Message::text("this is not an envelope")),
        ],
    )
    .await;
    let mut responses = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "outstanding".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    assert!(matches!(
        responses
            .next()
            .await
            .expect("the outstanding exchange ends")
            .expect_err("the peer sent a frame that is not an envelope"),
        WebSocketClientError::ExchangeProtocolViolation { .. }
    ));

    let refused_notification = endpoint
        .connection
        .notify(TypingNotification {
            who: "alice".to_string(),
        })
        .await
        .expect_err("a connection nobody reads carries nothing");

    assert!(matches!(
        refused_notification,
        WebSocketClientError::ExchangeProtocolViolation { .. }
    ));

    let refused_request = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "refused".to_string(),
        })
        .await
        .map(drop)
        .expect_err("a connection nobody reads accepts no exchange");

    assert!(matches!(
        refused_request,
        WebSocketClientError::ExchangeProtocolViolation { .. }
    ));

    let ScriptedPeerEndpoint { connection, peer } = endpoint;

    drop(responses);
    drop(connection);
    peer.finish().await;
}
