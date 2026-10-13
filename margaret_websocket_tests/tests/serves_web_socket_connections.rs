use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

use margaret_websocket_tests::driver_harness::DriverHarness;
use margaret_websocket_tests::test_dispatch_table::test_dispatch_table;

#[tokio::test]
async fn streams_a_request_across_many_frames_so_clients_see_progress_before_the_end() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":1,"method":"conversation_message","params":{"prompt":"cats"}}"#)
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
        .send(r#"{"id":"abc","method":"ping","params":{"label":"here"}}"#)
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
async fn applies_a_notification_to_the_session_so_fire_and_forget_messages_take_effect() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"method":"typing","params":{"who":"alice"}}"#)
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
async fn answers_a_request_whose_parameters_fail_validation_with_invalid_params_for_its_id() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":2,"method":"conversation_message","params":{"prompt":""}}"#)
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":2"));
    assert!(error.contains("invalid_params"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn answers_a_request_whose_parameters_do_not_deserialize_with_invalid_params_for_its_id() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":3,"method":"conversation_message","params":{"prompt":123}}"#)
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":3"));
    assert!(error.contains("invalid_params"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn reports_an_unknown_request_method_under_its_id_so_the_client_can_match_it() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":4,"method":"nonexistent","params":{}}"#)
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":4"));
    assert!(error.contains("unknown_method"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn ignores_an_unknown_notification_method_since_it_has_no_id_to_answer() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"method":"nonexistent","params":{}}"#)
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
async fn drops_a_notification_whose_parameters_fail_validation_since_it_has_no_id_to_answer() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"method":"typing","params":{"who":""}}"#)
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
async fn skips_binary_frames_and_keeps_serving_later_requests_on_the_connection() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send_raw(Message::Binary(vec![1, 2, 3].into()))
        .await;
    harness
        .send(r#"{"id":5,"method":"ping","params":{"label":"after binary"}}"#)
        .await;

    let response = harness.recv().await;

    assert!(response.contains("pong after binary"));

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn closes_on_a_malformed_frame_since_the_client_broke_the_message_protocol() {
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
async fn finishes_the_driver_when_cancelled_so_shutdown_closes_open_connections() {
    let harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness.cancellation_token.cancel();
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn continues_when_a_handler_fails_to_serialize_a_response() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":6,"method":"failing","params":{"prompt":"boom"}}"#)
        .await;
    let response: Value =
        serde_json::from_str(&harness.recv().await).expect("the error response is valid JSON");

    assert_eq!(
        response,
        serde_json::json!({
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
        .send(r#"{"id":7,"method":"flood","params":{"prompt":"forever"}}"#)
        .await;

    let _first = harness.recv().await;

    let DriverHarness { client, driver, .. } = harness;

    drop(client);
    driver
        .await
        .expect("the driver finishes after the client disconnects");
}
