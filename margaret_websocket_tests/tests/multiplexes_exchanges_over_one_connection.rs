use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_client::response_backlog_limit::RESPONSE_BACKLOG_LIMIT;
use margaret_websocket_client::response_item::ResponseItem;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;

const FIRST_EXCHANGE: u8 = 0;
const SECOND_EXCHANGE: u8 = 1;

fn chunk(id: u8) -> ScriptedPeerStep {
    ScriptedPeerStep::Send(Message::text(format!(
        r#"{{"id":{id},"done":false,"method":"response_chunk","result":{{"text":"chunk"}}}}"#
    )))
}

fn answer(id: u8) -> ScriptedPeerStep {
    ScriptedPeerStep::Send(Message::text(format!(
        r#"{{"id":{id},"done":true,"method":"response_chunk","result":{{"text":"answered"}}}}"#
    )))
}

fn script(chunks_for_the_first_exchange: usize) -> Vec<ScriptedPeerStep> {
    let mut steps = vec![
        ScriptedPeerStep::AwaitClientFrame,
        ScriptedPeerStep::AwaitClientFrame,
    ];

    for _ in 0..chunks_for_the_first_exchange {
        steps.push(chunk(FIRST_EXCHANGE));
    }

    steps.push(answer(SECOND_EXCHANGE));

    steps
}

async fn endpoint_with(chunks_for_the_first_exchange: usize) -> ScriptedPeerEndpoint {
    ScriptedPeerEndpoint::start(
        ScriptedPeerClosing::Cleanly,
        script(chunks_for_the_first_exchange),
    )
    .await
}

async fn request(
    endpoint: &ScriptedPeerEndpoint,
    label: &str,
) -> margaret_websocket_client::response_stream::ResponseStream<ResponseChunk> {
    endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: label.to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected")
}

#[tokio::test]
async fn serves_an_exchange_while_another_stream_goes_unread() {
    let endpoint = endpoint_with(2).await;
    let _unread = request(&endpoint, "unread").await;
    let mut answered = request(&endpoint, "answered").await;

    assert!(matches!(
        answered
            .next()
            .await
            .expect("the peer answers the exchange that is being drained")
            .expect("the answer reads as a response chunk"),
        ResponseItem::Payload(_)
    ));
}

#[tokio::test]
async fn interrupts_only_the_exchange_whose_consumer_stopped_draining() {
    let endpoint = endpoint_with(RESPONSE_BACKLOG_LIMIT + 1).await;
    let mut stalled = request(&endpoint, "stalled").await;
    let mut answered = request(&endpoint, "answered").await;

    assert!(matches!(
        answered
            .next()
            .await
            .expect("the peer answers the exchange that is being drained")
            .expect("the answer reads as a response chunk"),
        ResponseItem::Payload(_)
    ));

    let mut backlog = 0;

    let interruption = loop {
        match stalled.next().await.expect("the stalled exchange ends") {
            Ok(_) => backlog += 1,
            Err(error) => break error,
        }
    };

    assert_eq!(backlog, RESPONSE_BACKLOG_LIMIT);
    assert!(matches!(
        interruption,
        WebSocketClientError::ExchangeBacklogExceeded { .. }
    ));
}
