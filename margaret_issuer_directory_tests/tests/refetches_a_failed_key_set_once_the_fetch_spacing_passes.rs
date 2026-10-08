use std::sync::Arc;

use tokio::time::Instant;

use margaret_issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer;
use margaret_issuer_directory_tests::polled_directory::PolledDirectory;
use margaret_issuer_directory_tests::polled_fixture::PolledFixture;
use margaret_issuer_key_set::issuer_fetch_spacing::ISSUER_FETCH_SPACING;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;

#[tokio::test(start_paused = true)]
async fn refetches_a_failed_key_set_once_the_fetch_spacing_passes() {
    let polled = PolledFixture::published(JwksEndpointIssuer {
        issuer: "https://localhost",
        jwks_uri: "http://localhost/jwks",
    });
    let started_at = Instant::now();
    let directory = PolledDirectory::start(
        vec![Arc::clone(&polled.polled)],
        IssuerRequestClient::create().expect("the issuer request client builds"),
    );
    let first_fetch = polled.key_set.snapshot();

    polled.key_set.request_refresh_after(&first_fetch).await;

    let second_fetch = polled.key_set.snapshot();

    assert!(matches!(
        polled.key_set.request_refresh_after(&second_fetch).await,
        KeySetRefresh::Unchanged
    ));
    assert_eq!(started_at.elapsed(), ISSUER_FETCH_SPACING);

    directory.stop().await;
}
