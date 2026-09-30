use std::sync::Arc;

use margaret_issuer_directory_tests::failing_endpoint::FailingEndpoint;
use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test]
async fn keeps_awaiting_an_endpoint_that_cannot_be_provided() {
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(FailingEndpoint),
        Arc::new(localhost_trust()),
    ));

    assert!(matches!(
        first_poll(
            &trusted_issuer,
            IssuerRequestClient::create().expect("the issuer request client builds")
        )
        .await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));
}
