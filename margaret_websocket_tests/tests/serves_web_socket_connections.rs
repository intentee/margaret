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
    assert!(terminal.contains("\"done\":true"));
    assert!(terminal.contains("\"id\":1"));
    assert!(terminal.contains("\"method\":\"conversation_message\""));

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
async fn continues_when_a_handler_fails_to_serialize_a_response() {
    let mut harness = DriverHarness::spawn(test_dispatch_table()).await;

    harness
        .send(r#"{"id":6,"method":"failing","params":{"prompt":"boom"}}"#)
        .await;
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
