use std::sync::Arc;

use tokio::time::Instant;

use margaret_issuer_directory_tests::failing_endpoint::FailingEndpoint;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(start_paused = true)]
async fn refetches_a_failed_key_set_once_the_fetch_spacing_passes() {
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(FailingEndpoint),
        Arc::new(localhost_trust()),
    ));
    let started_at = Instant::now();
    let directory = PolledDirectory::start(
        vec![Arc::clone(&trusted_issuer)],
        IssuerRequestClient::create().expect("the issuer request client builds"),
    );
    let first_fetch = trusted_issuer.key_set.snapshot();

    trusted_issuer.key_set.refreshed_since(&first_fetch).await;

    let second_fetch = trusted_issuer.key_set.snapshot();

    assert!(matches!(
        trusted_issuer.key_set.refreshed_since(&second_fetch).await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));
    assert_eq!(started_at.elapsed(), ISSUER_FETCH_SPACING);

    directory.stop().await;
}
