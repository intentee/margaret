use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use url::Url;

use margaret_jwks_client::endpoint_key_set_locator::EndpointKeySetLocator;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;

#[tokio::test]
async fn endpoint_key_set_locator_locates_the_provided_endpoint() {
    let key_set_url =
        Url::parse("https://issuer.example/.well-known/jwks.json").expect("the url parses");
    let locator = EndpointKeySetLocator {
        endpoint_provider: Arc::new(StaticEndpoint::new(key_set_url.clone())),
    };

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &CancellationToken::new(),
            issuer_document_client: &system_issuer_document_client(),
            timeout: KEY_SET_POLL_INTERVAL_BEFORE_READY,
        })
        .await;

    assert!(matches!(location, KeySetLocation::Located(located) if located == key_set_url));
}
