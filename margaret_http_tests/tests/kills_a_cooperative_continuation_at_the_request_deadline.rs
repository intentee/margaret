use std::sync::Arc;

use tokio::sync::mpsc::unbounded_channel;

use margaret_http::request_cancellation_cooperation::RequestCancellationCooperation;
use margaret_http_tests::brief_request_timeout::brief_request_timeout;
use margaret_http_tests::open_plain_request::open_plain_request;
use margaret_http_tests::responder_outcome::ResponderOutcome;
use margaret_http_tests::running_plain_server::RunningPlainServer;
use margaret_http_tests::uncooperative_responder::UncooperativeResponder;

#[tokio::test]
async fn kills_a_cooperative_continuation_at_the_request_deadline() {
    let (outcomes, mut observed_outcomes) = unbounded_channel();
    let (started, mut start_observed) = unbounded_channel();
    let server = RunningPlainServer::start(
        Arc::new(UncooperativeResponder { outcomes, started }),
        RequestCancellationCooperation::Cooperative,
        brief_request_timeout(),
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
        Some(ResponderOutcome::Dropped),
        "a continuation that never observes its token is still released at the deadline"
    );

    server.stop().await;
}
