use std::sync::Arc;

use reqwest::StatusCode;
use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_key_set_poll::key_set_poll::KeySetPoll;
use margaret_key_set_poll::key_set_poll_failure::KeySetPollFailure;
use margaret_key_set_poll::key_set_poll_interval_before_ready::KEY_SET_POLL_INTERVAL_BEFORE_READY;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::fixed_key_set_locator::FixedKeySetLocator;

#[tokio::test]
async fn key_set_poll_service_reports_an_error_status_of_the_issuer() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/jwks",
            vec![MethodHandler::anonymous(
                "GET",
                Arc::new(StaticHandler {
                    body: br#"{"keys":[]}"#.to_vec(),
                    content_type: "application/json",
                    status: 503,
                }),
            )],
        )],
    )
    .await;
    let service = KeySetPollService {
        issuer_document_client: IssuerDocumentClient::build(fixture_client_builder(
            &fixture.certificate_authority,
        ))
        .expect("the issuer document client builds"),
        locator: FixedKeySetLocator {
            key_set_url: fixture.url(server.port(), "/jwks"),
        },
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };

    let poll = service
        .fetch_key_set(
            &CancellationToken::new(),
            KEY_SET_POLL_INTERVAL_BEFORE_READY,
        )
        .await;

    assert!(matches!(
        poll,
        KeySetPoll::Failed(KeySetPollFailure::DocumentStatus(
            StatusCode::SERVICE_UNAVAILABLE
        ))
    ));

    server.stop().await;
}
