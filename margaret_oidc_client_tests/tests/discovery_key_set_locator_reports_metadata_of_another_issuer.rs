use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http_tests::static_handler::StaticHandler;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_key_set_poll::key_set_location::KeySetLocation;
use margaret_key_set_poll::key_set_location_request::KeySetLocationRequest;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::locates_key_set::LocatesKeySet as _;
use margaret_oidc_client::discovery_failure::DiscoveryFailure;
use margaret_oidc_client::discovery_key_set_locator::DiscoveryKeySetLocator;
use margaret_oidc_client_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_oidc_discovery::oidc_discovery_url::oidc_discovery_url;
use margaret_oidc_discovery::provider_metadata_rejection::ProviderMetadataRejection;

#[tokio::test(flavor = "multi_thread")]
async fn discovery_key_set_locator_reports_metadata_of_another_issuer() {
    let trust = localhost_trust();
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(StaticHandler {
            body: br#"{"issuer":"https://attacker.example","jwks_uri":"https://attacker.example/jwks"}"#.to_vec(),
            content_type: "application/json",
            status: 200,
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
        panic!("metadata of another issuer is not located");
    };

    assert!(
        failure
            .to_string()
            .starts_with("the provider metadata is rejected: ")
    );
    assert!(matches!(
        failure,
        DiscoveryFailure::MetadataRejected(ProviderMetadataRejection::IssuerMismatch { .. })
    ));

    issuer.stop().await;
}
