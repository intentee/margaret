use std::sync::Arc;

use rustls::ClientConfig;
use rustls::RootCertStore;
use rustls::crypto::aws_lc_rs;
use tokio_util::sync::CancellationToken;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_request::issuer_document::IssuerDocument;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_route_method::route_method::RouteMethod;

#[tokio::test]
async fn uses_openssl_even_when_rustls_is_preconfigured() {
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/document",
            vec![MethodHandler::anonymous(
                RouteMethod::Get,
                Arc::new(StaticHandler {
                    body: br#"{"keys":[]}"#.to_vec(),
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let rustls_trusting_nothing =
        ClientConfig::builder_with_provider(Arc::new(aws_lc_rs::default_provider()))
            .with_safe_default_protocol_versions()
            .expect("the default provider supports the safe protocol versions")
            .with_root_certificates(RootCertStore::empty())
            .with_no_client_auth();
    let client = IssuerRequestClient::build(
        fixture_client_builder(&fixture.certificate_authority)
            .use_preconfigured_tls(rustls_trusting_nothing),
    )
    .expect("the issuer request client builds");

    let fetched = client
        .fetch_document(
            fixture.url(server.port(), "/document"),
            &CancellationToken::new(),
        )
        .await;

    assert!(matches!(fetched, IssuerDocument::Fetched(_)));

    server.stop().await;
}
