use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_key_set_poll_tests::system_issuer_document_client::system_issuer_document_client;
use margaret_oidc_client::discovery_key_set_locator::DiscoveryKeySetLocator;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use url::Url;

#[tokio::test]
async fn discovery_key_set_locator_reports_an_unreachable_issuer() {
    let locator = DiscoveryKeySetLocator {
        discovery_url: Url::parse("https://127.0.0.1:1/.well-known/openid-configuration")
            .expect("the unreachable url parses"),
        token_trust: Arc::new(localhost_trust()),
    };

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &CancellationToken::new(),
            issuer_document_client: &system_issuer_document_client(),
            timeout: KEY_SET_POLL_INTERVAL_BEFORE_READY,
        })
        .await;

    let KeySetLocation::Failed(failure) = location else {
        panic!("an unreachable issuer is not located");
    };

    assert!(
        failure
            .to_string()
            .starts_with("the provider metadata could not be transferred from the issuer: ")
    );
}
