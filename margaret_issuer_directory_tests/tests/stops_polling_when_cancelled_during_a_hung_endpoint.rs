use std::sync::Arc;

use margaret_issuer_directory_tests::hanging_endpoint::HangingEndpoint;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn stops_polling_when_cancelled_during_a_hung_endpoint() {
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(HangingEndpoint),
        Arc::new(localhost_trust()),
    ));
    let snapshot = trusted_issuer.key_set.snapshot();

    PolledDirectory::start(
        vec![Arc::clone(&trusted_issuer)],
        IssuerRequestClient::create().expect("the issuer request client builds"),
    )
    .stop()
    .await;

    assert!(matches!(
        trusted_issuer.key_set.refreshed_since(&snapshot).await,
        KeySetRefresh::PollingStopped
    ));
}
