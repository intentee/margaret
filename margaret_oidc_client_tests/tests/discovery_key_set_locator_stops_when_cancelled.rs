use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;
use margaret_oidc_client::discovery_key_set_locator::DiscoveryKeySetLocator;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;

#[tokio::test]
async fn discovery_key_set_locator_stops_when_cancelled() {
    let trust = localhost_trust();
    let locator = DiscoveryKeySetLocator {
        discovery_url: oidc_discovery_url(&trust.issuer),
        token_trust: Arc::new(trust),
    };
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &cancellation_token,
            issuer_document_client: &system_issuer_document_client(),
            timeout: KEY_SET_POLL_INTERVAL_BEFORE_READY,
        })
        .await;

    assert!(matches!(location, KeySetLocation::Cancelled));
}
