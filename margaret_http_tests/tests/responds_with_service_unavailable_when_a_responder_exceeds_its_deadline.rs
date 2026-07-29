use std::sync::Arc;

use tokio::sync::mpsc::unbounded_channel;

use margaret_http::request_cancellation_cooperation::RequestCancellationCooperation;
use margaret_http_tests::brief_request_timeout::brief_request_timeout;
use margaret_http_tests::plain_client_request::plain_client_request;
use margaret_http_tests::running_plain_server::RunningPlainServer;
use margaret_http_tests::uncooperative_responder::UncooperativeResponder;

#[tokio::test]
async fn responds_with_service_unavailable_when_a_responder_exceeds_its_deadline() {
    let (outcomes, _observed_outcomes) = unbounded_channel();
    let (started, _start_observed) = unbounded_channel();
    let server = RunningPlainServer::start(
        Arc::new(UncooperativeResponder { outcomes, started }),
        RequestCancellationCooperation::Immediate,
        brief_request_timeout(),
    )
    .await;

    let response = plain_client_request(server.address()).await;

    assert!(response.contains(" 503 "));

    server.stop().await;
}
