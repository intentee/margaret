use serde_json::Value;
use serde_json::json;

use margaret_websocket_client::response_credit_window::RESPONSE_CREDIT_WINDOW;
use margaret_websocket_envelope::client_sent_frame::ClientSentFrame;
use margaret_websocket_envelope::credit_grant::CreditGrant;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_tests::client_request_frame::client_request_frame;
use margaret_websocket_tests::driver_harness::DriverHarness;
use margaret_websocket_tests::ping_message::PingMessage;
use margaret_websocket_tests::response_chunk::ResponseChunk;
use margaret_websocket_tests::scripted_peer_closing::ScriptedPeerClosing;
use margaret_websocket_tests::scripted_peer_endpoint::ScriptedPeerEndpoint;
use margaret_websocket_tests::scripted_peer_step::ScriptedPeerStep;
use margaret_websocket_tests::scripted_response_chunk::scripted_response_chunk;
use margaret_websocket_tests::test_dispatch_table::test_dispatch_table;

/// A request that grants no window parks its handler before it emits a frame.
const EXHAUSTED_WINDOW: CreditGrant = CreditGrant::from_frames(0);
const PARKED_EXCHANGE: i64 = 8;

fn cancel_frame(id: RequestId) -> String {
    serde_json::to_string(&ClientSentFrame::<()>::Cancel { id }).expect("a cancel frame serializes")
}

fn parked_request() -> String {
    client_request_frame(
        EXHAUSTED_WINDOW,
        RequestId::Number(PARKED_EXCHANGE),
        "flood",
        json!({"prompt": "forever"}),
    )
}

fn answered_request() -> String {
    client_request_frame(
        RESPONSE_CREDIT_WINDOW,
        RequestId::Number(PARKED_EXCHANGE),
        "ping",
        json!({"label": "reclaimed"}),
    )
}

#[tokio::test]
async fn reclaims_the_request_id_of_a_cancelled_exchange() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.send(&parked_request()).await;
    harness.send(&answered_request()).await;

    let refused: Value =
        serde_json::from_str(&harness.recv().await).expect("the error response is valid JSON");

    assert_eq!(
        refused,
        json!({
            "kind": "error",
            "error": {
                "code": "duplicate_request_id",
                "details": null,
                "message": "another exchange is already open under this request id"
            },
            "id": PARKED_EXCHANGE
        })
    );

    harness
        .send(&cancel_frame(RequestId::Number(PARKED_EXCHANGE)))
        .await;
    harness.send(&answered_request()).await;

    assert!(harness.recv().await.contains("pong reclaimed"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn tells_the_peer_that_an_abandoned_exchange_is_over() {
    let endpoint = ScriptedPeerEndpoint::start(
        ScriptedPeerClosing::Cleanly,
        vec![
            ScriptedPeerStep::AwaitClientFrame,
            scripted_response_chunk(RequestId::Number(0), false, "chunk"),
            ScriptedPeerStep::AwaitClientCancel,
        ],
    )
    .await;
    let responses = endpoint
        .connection
        .request::<PingMessage, ResponseChunk>(PingMessage {
            label: "abandoned".to_string(),
        })
        .await
        .expect("the request reaches a peer that is still connected");

    drop(responses);

    endpoint.peer.finish().await;
}
