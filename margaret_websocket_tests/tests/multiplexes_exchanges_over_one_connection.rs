use margaret_websocket_client::exchange_queue_capacity::EXCHANGE_QUEUE_CAPACITY;
use margaret_websocket_client::response_credit_window::RESPONSE_CREDIT_WINDOW;
use margaret_websocket_client::response_item::ResponseItem;
use margaret_websocket_client::response_stream::ResponseStream;
use margaret_websocket_client::web_socket_client_error::WebSocketClientError;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;
use margaret_websocket_tests::scripted_response_chunk::scripted_response_chunk;
use margaret_websocket_tests::scripted_response_rejection::scripted_response_rejection;

const FIRST_EXCHANGE: i64 = 0;
const SECOND_EXCHANGE: i64 = 1;

fn chunk(id: i64) -> ScriptedPeerStep {
    scripted_response_chunk(RequestId::Number(id), false, "chunk")
}

fn answer(id: i64) -> ScriptedPeerStep {
    scripted_response_chunk(RequestId::Number(id), true, "answered")
}

fn awaited_requests(count: usize) -> Vec<ScriptedPeerStep> {
    (0..count)
        .map(|_| ScriptedPeerStep::AwaitClientFrame)
        .collect()
}

async fn endpoint(script: Vec<ScriptedPeerStep>) -> ScriptedPeerEndpoint {
    ScriptedPeerEndpoint::start(ScriptedPeerClosing::Cleanly, script).await
}

async fn request(endpoint: &ScriptedPeerEndpoint, label: &str) -> ResponseStream<ResponseChunk> {
    endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: label.to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected")
}

#[derive(Debug)]
struct DrainedExchange {
    delivered: usize,
    interruption: Option<WebSocketClientError>,
}

async fn drain(mut responses: ResponseStream<ResponseChunk>) -> DrainedExchange {
    let mut delivered = 0;

    while let Some(item) = responses.next().await {
        match item {
            Ok(_) => delivered += 1,
            Err(interruption) => {
                return DrainedExchange {
                    delivered,
                    interruption: Some(interruption),
                };
            }
        }
    }

    DrainedExchange {
        delivered,
        interruption: None,
    }
}

#[tokio::test]
async fn serves_an_exchange_while_another_stream_goes_unread() {
    let endpoint = endpoint(
        awaited_requests(2)
            .into_iter()
            .chain([chunk(FIRST_EXCHANGE), chunk(FIRST_EXCHANGE)])
            .chain([answer(SECOND_EXCHANGE)])
            .collect(),
    )
    .await;
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
async fn delivers_a_burst_that_fills_the_whole_window() {
    let endpoint = endpoint(
        awaited_requests(1)
            .into_iter()
            .chain((1..RESPONSE_CREDIT_WINDOW).map(|_| chunk(FIRST_EXCHANGE)))
            .chain([answer(FIRST_EXCHANGE)])
            .collect(),
    )
    .await;
    let responses = request(&endpoint, "burst").await;
    let DrainedExchange {
        delivered,
        interruption,
    } = drain(responses).await;

    assert!(interruption.is_none(), "{interruption:?}");
    assert_eq!(delivered, RESPONSE_CREDIT_WINDOW);
}

#[tokio::test]
async fn delivers_a_rejection_that_arrives_once_the_window_is_spent() {
    let endpoint = endpoint(
        awaited_requests(1)
            .into_iter()
            .chain((0..RESPONSE_CREDIT_WINDOW).map(|_| chunk(FIRST_EXCHANGE)))
            .chain([scripted_response_rejection(
                RequestId::Number(FIRST_EXCHANGE),
                "the handler gave up",
            )])
            .collect(),
    )
    .await;
    let responses = request(&endpoint, "rejected").await;
    let DrainedExchange {
        delivered,
        interruption,
    } = drain(responses).await;

    assert!(interruption.is_none(), "{interruption:?}");
    assert_eq!(delivered, EXCHANGE_QUEUE_CAPACITY);
}

#[tokio::test]
async fn replenishes_the_window_as_the_consumer_drains() {
    let endpoint = endpoint(vec![
        ScriptedPeerStep::AwaitClientFrame,
        chunk(FIRST_EXCHANGE),
        ScriptedPeerStep::AwaitClientCredit,
        answer(FIRST_EXCHANGE),
    ])
    .await;
    let responses = request(&endpoint, "drained").await;
    let DrainedExchange {
        delivered,
        interruption,
    } = drain(responses).await;

    endpoint.peer.finish().await;

    assert!(interruption.is_none(), "{interruption:?}");
    assert_eq!(delivered, 2);
}

#[tokio::test]
async fn reports_a_peer_that_sends_past_the_window_it_was_granted() {
    let endpoint = endpoint(
        awaited_requests(2)
            .into_iter()
            .chain((0..=EXCHANGE_QUEUE_CAPACITY).map(|_| chunk(FIRST_EXCHANGE)))
            .chain([answer(SECOND_EXCHANGE)])
            .collect(),
    )
    .await;
    let stalled = request(&endpoint, "stalled").await;
    let mut answered = request(&endpoint, "answered").await;

    assert!(matches!(
        answered
            .next()
            .await
            .expect("the peer answers the exchange that is being drained")
            .expect("the answer reads as a response chunk"),
        ResponseItem::Payload(_)
    ));

    let DrainedExchange {
        delivered,
        interruption,
    } = drain(stalled).await;

    assert_eq!(delivered, EXCHANGE_QUEUE_CAPACITY);
    assert!(matches!(
        interruption.expect("a peer that overruns the window ends the exchange"),
        WebSocketClientError::ExchangeCreditExceeded { credit, .. }
            if credit == RESPONSE_CREDIT_WINDOW
    ));
}
