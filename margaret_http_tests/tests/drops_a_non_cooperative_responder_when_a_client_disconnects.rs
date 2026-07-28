use std::sync::Arc;

use tokio::sync::mpsc::unbounded_channel;

use margaret_http::request_cancellation_cooperation::RequestCancellationCooperation;
use margaret_http_tests::cancellation_recording_responder::CancellationRecordingResponder;
use margaret_http_tests::open_plain_request::open_plain_request;
use margaret_http_tests::responder_outcome::ResponderOutcome;
use margaret_http_tests::running_plain_server::RunningPlainServer;

#[tokio::test]
async fn drops_a_non_cooperative_responder_when_a_client_disconnects() {
    let (outcomes, mut observed_outcomes) = unbounded_channel();
    let (started, mut start_observed) = unbounded_channel();
    let server = RunningPlainServer::start(
        Arc::new(CancellationRecordingResponder { outcomes, started }),
        RequestCancellationCooperation::Immediate,
    )
    .await;

    let request = open_plain_request(server.address()).await;

    start_observed
        .recv()
        .await
        .expect("the responder starts before the client disconnects");

    drop(request);

    assert_eq!(
        observed_outcomes.recv().await,
        Some(ResponderOutcome::Dropped)
    );

    server.stop().await;
}
