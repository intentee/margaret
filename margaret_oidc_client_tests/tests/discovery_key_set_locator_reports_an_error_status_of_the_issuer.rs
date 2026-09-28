use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_oidc_client::discovery_key_set_locator::DiscoveryKeySetLocator;
use margaret_oidc_client_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;

#[tokio::test(flavor = "multi_thread")]
async fn discovery_key_set_locator_reports_an_error_status_of_the_issuer() {
    let trust = localhost_trust();
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(StaticHandler {
            body: br"{}".to_vec(),
            content_type: "application/json",
            status: 503,
        }),
        key_set: Arc::new(StaticHandler {
            body: br#"{"keys":[]}"#.to_vec(),
            content_type: "application/json",
            status: 200,
        }),
    })
    .await;
    let locator = DiscoveryKeySetLocator {
        discovery_url: oidc_discovery_url(&trust.issuer),
        token_trust: Arc::new(trust),
    };
    let issuer_document_client = IssuerDocumentClient::build(issuer.client_builder())
        .expect("the issuer document client builds");

    let location = locator
        .locate(KeySetLocationRequest {
            cancellation_token: &CancellationToken::new(),
            issuer_document_client: &issuer_document_client,
            timeout: KEY_SET_POLL_INTERVAL_BEFORE_READY,
        })
        .await;

    let KeySetLocation::Failed(failure) = location else {
        panic!("an error status is not located");
    };

    assert_eq!(
        failure.to_string(),
        "the issuer answered the discovery request with status 503 Service Unavailable"
    );

    issuer.stop().await;
}
