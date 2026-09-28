use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_jwks_client::jwks_client_error::JwksClientError;
use margaret_jwks_client::jwks_poll_interval_before_ready::JWKS_POLL_INTERVAL_BEFORE_READY;
use margaret_jwks_client::public_jwks_poll::PublicJwksPoll;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client::verification_key_set_holder::VerificationKeySetHolder;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;

#[tokio::test]
async fn public_jwks_poll_service_reports_a_document_that_is_not_a_key_set() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            WELL_KNOWN_JWKS_PATH,
            vec![MethodHandler::anonymous(
                "GET",
                Arc::new(StaticHandler {
                    body: br#"{"issuer":"https://localhost"}"#.to_vec(),
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let service = PublicJwksPollService {
        endpoint_provider: Arc::new(StaticEndpoint::new(
            fixture.url(server.port(), WELL_KNOWN_JWKS_PATH),
        )),
        issuer_document_client: IssuerDocumentClient::build(fixture_client_builder(
            &fixture.certificate_authority,
        ))
        .expect("the issuer document client builds"),
        verification_key_set_holder: VerificationKeySetHolder::default(),
    };

    let poll = service
        .fetch_public_jwks(&CancellationToken::new(), JWKS_POLL_INTERVAL_BEFORE_READY)
        .await;

    assert!(matches!(
        poll,
        PublicJwksPoll::Failed(JwksClientError::DocumentRejected { .. })
    ));

    server.stop().await;
}
