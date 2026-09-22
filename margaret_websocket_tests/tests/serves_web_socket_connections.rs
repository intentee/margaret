use serde_json::Value;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_client::response_credit_window::RESPONSE_CREDIT_WINDOW;
use margaret_websocket_envelope::credit_grant::CreditGrant;
use margaret_websocket_envelope::request_id::RequestId;
use margaret_websocket_tests::client_notification_frame::client_notification_frame;
use margaret_websocket_tests::client_request_frame::client_request_frame;
use margaret_websocket_tests::driver_harness::DriverHarness;
use margaret_websocket_tests::test_dispatch_table::test_dispatch_table;

/// A request that grants no window parks its handler before it emits a frame.
const EXHAUSTED_WINDOW: CreditGrant = CreditGrant::from_frames(0);

#[tokio::test]
async fn streams_a_request_across_many_frames() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(1),
            "conversation_message",
            json!({"prompt": "cats"}),
        ))
        .await;

    let chunk = harness.recv().await;
    let terminal = harness.recv().await;

    assert!(chunk.contains("thinking about cats"));
    assert!(chunk.contains("\"done\":false"));
    assert!(chunk.contains("\"method\":\"response_chunk\""));
    assert!(terminal.contains("\"done\":true"));
    assert!(terminal.contains("\"id\":1"));
    assert!(terminal.contains("\"method\":\"storyboard_complete\""));
    assert!(!chunk.contains("\"method\":\"conversation_message\""));
    assert!(!terminal.contains("\"method\":\"conversation_message\""));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn answers_a_single_response_request_with_a_string_id() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Text("abc".to_owned()),
            "ping",
            json!({"label": "here"}),
        ))
        .await;

    let response = harness.recv().await;

    assert!(response.contains("pong here"));
    assert!(response.contains("\"id\":\"abc\""));
    assert!(response.contains("\"done\":true"));
    assert!(response.contains("\"method\":\"response_chunk\""));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn applies_a_notification_to_the_session() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_notification_frame(
            "typing",
            json!({"who": "alice"}),
        ))
        .await;
    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");

    let notifications = harness
        .session
        .notifications
        .lock()
        .expect("the notification log is not poisoned");

    assert_eq!(notifications.as_slice(), ["alice"]);
}

#[tokio::test]
async fn reports_invalid_request_parameters() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(2),
            "conversation_message",
            json!({"prompt": ""}),
        ))
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":2"));
    assert!(error.contains("invalid_params"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn reports_malformed_request_parameters() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(3),
            "conversation_message",
            json!({"prompt": 123}),
        ))
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":3"));
    assert!(error.contains("invalid_params"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn reports_an_unknown_request_method() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(4),
            "nonexistent",
            json!({}),
        ))
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":4"));
    assert!(error.contains("unknown_method"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn ignores_an_unknown_notification_method() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_notification_frame("nonexistent", json!({})))
        .await;
    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");

    assert!(
        harness
            .session
            .notifications
            .lock()
            .expect("the notification log is not poisoned")
            .is_empty()
    );
}

#[tokio::test]
async fn ignores_invalid_notification_parameters() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_notification_frame("typing", json!({"who": ""})))
        .await;
    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");

    assert!(
        harness
            .session
            .notifications
            .lock()
            .expect("the notification log is not poisoned")
            .is_empty()
    );
}

#[tokio::test]
async fn ignores_non_text_frames() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send_raw(Message::Binary(vec![1, 2, 3].into()))
        .await;
    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(5),
            "ping",
            json!({"label": "after binary"}),
        ))
        .await;

    let response = harness.recv().await;

    assert!(response.contains("pong after binary"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn closes_on_a_malformed_frame() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.send("this is not a websocket envelope").await;
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn closes_when_the_client_sends_a_close_frame() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn closes_on_cancellation() {
    let harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn continues_when_a_handler_fails_to_serialize_a_response() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(6),
            "failing",
            json!({"prompt": "boom"}),
        ))
        .await;
    let response: Value =
        serde_json::from_str(&harness.recv().await).expect("the error response is valid JSON");

    assert_eq!(
        response,
        json!({
            "kind": "error",
            "error": {
                "code": "internal_error",
                "details": null,
                "message": "Internal error"
            },
            "id": 6
        })
    );
    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn stops_when_the_client_disconnects_mid_stream() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(&client_request_frame(
            RESPONSE_CREDIT_WINDOW,
            RequestId::Number(7),
            "flood",
            json!({"prompt": "forever"}),
        ))
        .await;

    let _first = harness.recv().await;

    let DriverHarness { client, driver, .. } = harness;

    drop(client);
    driver
        .await
        .expect("the driver finishes after the client disconnects");
}

#[tokio::test]
async fn reports_a_request_id_that_is_already_open() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    for _ in 0..2 {
        harness
            .send(&client_request_frame(
                EXHAUSTED_WINDOW,
                RequestId::Number(8),
                "flood",
                json!({"prompt": "forever"}),
            ))
            .await;
    }

    let response: Value =
        serde_json::from_str(&harness.recv().await).expect("the error response is valid JSON");

    assert_eq!(
        response,
        json!({
            "kind": "error",
            "error": {
                "code": "duplicate_request_id",
                "details": null,
                "message": "another exchange is already open under this request id"
            },
            "id": 8
        })
    );

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}
