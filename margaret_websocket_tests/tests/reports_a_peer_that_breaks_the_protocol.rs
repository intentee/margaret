use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;

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
