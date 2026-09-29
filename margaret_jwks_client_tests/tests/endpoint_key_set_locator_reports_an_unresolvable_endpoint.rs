use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_jwks_client::endpoint_key_set_locator::EndpointKeySetLocator;
use margaret_jwks_client_tests::failing_endpoint::FailingEndpoint;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_fetch_timeout::KEY_SET_POLL_FETCH_TIMEOUT;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test]
async fn endpoint_key_set_locator_reports_an_unresolvable_endpoint() {
    let locator = EndpointKeySetLocator {
        endpoint_provider: Arc::new(FailingEndpoint),
    };

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &CancellationToken::new(),
            issuer_document_client: &system_issuer_document_client(),
            timeout: KEY_SET_POLL_FETCH_TIMEOUT,
        })
        .await;

    assert!(matches!(location, KeySetLocation::Failed(_)));
}
