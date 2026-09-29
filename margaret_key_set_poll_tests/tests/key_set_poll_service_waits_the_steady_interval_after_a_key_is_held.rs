use std::sync::Arc;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jws_verification::key_set_assembly::KeySetAssembly;
use margaret_jws_verification::verification_key_set::VerificationKeySet;
use margaret_key_set_poll::key_set_poll_interval_after_ready::KEY_SET_POLL_INTERVAL_AFTER_READY;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::failing_key_set_locator::FailingKeySetLocator;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test(start_paused = true)]
async fn key_set_poll_service_waits_the_steady_interval_after_a_key_is_held() {
    let KeySetAssembly::Assembled(known_good) = VerificationKeySet::assemble(Vec::new()) else {
        panic!("an empty key set is accepted");
    };
    let verification_key_set_holder = VerificationKeySetHolder::default();

    verification_key_set_holder.set(Some(Arc::new(known_good)));

    let mut service = KeySetPollService {
        issuer_document_client: system_issuer_document_client(),
        locator: FailingKeySetLocator,
        verification_key_set_holder,
    };
    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), KEY_SET_POLL_INTERVAL_AFTER_READY);
}
