use std::sync::Arc;

use url::Url;

use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_http_tests::truncated_response_server::TruncatedResponseServer;
use margaret_issuer_directory_tests::first_poll::first_poll;
use margaret_issuer_directory_tests::localhost_trust::localhost_trust;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_issuer_request::issuer_request_client::IssuerRequestClient;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwt_verification_tests::token_trust_declaration::TokenTrustDeclaration;
use margaret_trusted_issuer::trusted_issuer::TrustedIssuer;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_a_key_set_document_that_cannot_be_transferred() {
    let fixture = TlsFixture::generate();
    let server = TruncatedResponseServer::start(fixture.server_config.clone()).await;
    let trusted_issuer = Arc::new(TrustedIssuer::for_jwks_endpoint(
        Arc::new(StaticEndpoint::new(
            Url::parse(fixture.url(server.port(), "/jwks").as_str())
                .expect("the fixture url parses"),
        )),
        Arc::new(TokenTrustDeclaration {
            trust: localhost_trust(),
        }),
    ));

    assert!(matches!(
        first_poll(
            &trusted_issuer,
            IssuerRequestClient::build(fixture_client_builder(&fixture.certificate_authority))
                .expect("the fixture request client builds")
        )
        .await,
        KeySetRefresh::Refreshed(KeySetHolding::Awaiting)
    ));

    server.finish().await;
}
