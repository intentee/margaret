use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_jwks_client::endpoint_key_set_locator::EndpointKeySetLocator;
use margaret_jwks_client_tests::hanging_endpoint::HangingEndpoint;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_fetch_timeout::KEY_SET_POLL_FETCH_TIMEOUT;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test]
async fn endpoint_key_set_locator_stops_resolving_when_cancelled() {
    let locator = EndpointKeySetLocator {
        endpoint_provider: Arc::new(HangingEndpoint),
    };
    let cancellation_token = CancellationToken::new();

    cancellation_token.cancel();

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &cancellation_token,
            issuer_document_client: &system_issuer_document_client(),
            timeout: KEY_SET_POLL_FETCH_TIMEOUT,
        })
        .await;

    assert!(matches!(location, KeySetLocation::Cancelled));
}
