use margaret_websocket_tests::driver_harness::DriverHarness;
use margaret_websocket_tests::test_dispatch_table::test_dispatch_table;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn streams_a_request_across_many_frames() {
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
async fn applies_a_notification_to_the_session() {
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
async fn continues_after_a_notification_handler_fails() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"method":"failing_notification","params":{"who":"alice"}}"#)
        .await;
    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");

    let notifications = harness
        .session
        .notifications
        .lock()
        .expect("the notification log is not poisoned");

    assert!(notifications.is_empty());
}

#[tokio::test]
async fn reports_invalid_request_parameters() {
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
async fn reports_malformed_request_parameters() {
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
async fn reports_an_unknown_request_method() {
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
async fn ignores_an_unknown_notification_method() {
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
async fn ignores_invalid_notification_parameters() {
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
async fn ignores_non_text_frames() {
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
async fn sends_an_error_frame_when_a_request_handler_fails() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":6,"method":"failing","params":{"prompt":"boom"}}"#)
        .await;

    let error = harness.recv().await;

    assert!(error.contains("\"id\":6"));
    assert!(error.contains("internal_error"));

    harness.send_raw(Message::Close(None)).await;
    harness.driver.await.expect("the driver finishes");
}

#[tokio::test]
async fn does_not_send_a_second_terminal_after_the_handler_answered() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":8,"method":"respond_then_fail","params":{"prompt":"question"}}"#)
        .await;

    let answered = harness.recv().await;

    assert!(answered.contains("\"id\":8"));
    assert!(answered.contains("\"done\":true"));
    assert!(!answered.contains("internal_error"));

    harness
        .send(r#"{"id":9,"method":"ping","params":{"label":"after"}}"#)
        .await;

    let next = harness.recv().await;

    assert!(next.contains("\"id\":9"));
    assert!(!next.contains("internal_error"));

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
