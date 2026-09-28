use std::time::Duration;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::failing_key_set_locator::FailingKeySetLocator;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test(start_paused = true)]
async fn key_set_poll_service_does_not_wait_before_the_first_key_is_fetched() {
    let mut service = KeySetPollService {
        issuer_document_client: system_issuer_document_client(),
        locator: FailingKeySetLocator,
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), Duration::ZERO);
}
