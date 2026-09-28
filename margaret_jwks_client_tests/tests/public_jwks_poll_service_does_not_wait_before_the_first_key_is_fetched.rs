use std::sync::Arc;
use std::time::Duration;

use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jwks_client_tests::failing_endpoint::FailingEndpoint;
use margaret_jwks_client_tests::first_tick_context::first_tick_context;
use margaret_jwks_client_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test(start_paused = true)]
async fn public_jwks_poll_service_does_not_wait_before_the_first_key_is_fetched() {
    let mut service = PublicJwksPollService {
        endpoint_provider: Arc::new(FailingEndpoint),
        issuer_document_client: system_issuer_document_client(),
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };

    let started_at = Instant::now();

    service
        .handle_tick(CancellationToken::new(), first_tick_context())
        .await
        .expect("a failed poll never fails the service");

    assert_eq!(started_at.elapsed(), Duration::ZERO);
}
