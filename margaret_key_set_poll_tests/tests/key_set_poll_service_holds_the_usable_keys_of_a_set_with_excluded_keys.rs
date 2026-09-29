use std::sync::Arc;

use serde_json::json;
use tokio_util::sync::CancellationToken;
use trzcina::Ticker as _;

use margaret_http::method_handler::MethodHandler;
use margaret_http::route_entry::RouteEntry;
use margaret_http_tests::fixture_client_builder::fixture_client_builder;
use margaret_http_tests::running_fixture_server::RunningFixtureServer;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_http_tests::tls_fixture::TlsFixture;
use margaret_issuer_document_fetch::issuer_document_client::IssuerDocumentClient;
use margaret_jose_parameters::curve::Curve;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::jws_verification::JwsVerification;
use margaret_jws_verification_tests::fixture_key::FixtureKey;
use margaret_key_set_poll::key_set_poll_service::KeySetPollService;
use margaret_key_set_poll::verification_key_set_holder::VerificationKeySetHolder;
use margaret_key_set_poll_tests::first_tick_context::first_tick_context;
use margaret_key_set_poll_tests::fixed_key_set_locator::FixedKeySetLocator;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

#[tokio::test]
async fn key_set_poll_service_holds_the_usable_keys_of_a_set_with_excluded_keys() {
    let key = FixtureKey::generate(Curve::P256, "sig-kid");
    let document = json!({ "keys": [
        { "kty": "OKP", "crv": "Ed25519", "kid": "edwards-kid", "x": "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo" },
        { "kty": "oct", "kid": "secret-kid", "k": "c2VjcmV0" },
        key.jwk(),
    ] });
    let fixture = TlsFixture::generate();
    let server = RunningFixtureServer::start(
        fixture.server_config.clone(),
        vec![RouteEntry::new(
            "/jwks",
            vec![MethodHandler::anonymous(
                "GET",
                Arc::new(StaticHandler {
                    body: document.to_string().into_bytes(),
                    content_type: "application/json",
                    status: 200,
                }),
            )],
        )],
    )
    .await;
    let verification_key_set_holder = VerificationKeySetHolder::default();
    let mut subscription = verification_key_set_holder.subscribe();
    let mut service = KeySetPollService {
        issuer_document_client: IssuerDocumentClient::build(
            fixture_client_builder(&fixture.certificate_authority)
                .resolve(&fixture.server_name, server.address()),
        )
        .expect("the issuer document client builds"),
        locator: FixedKeySetLocator {
            key_set_url: fixture.url(server.port(), "/jwks"),
        },
        verification_key_set_holder: verification_key_set_holder.clone(),
    };

    let cancellation_token = CancellationToken::new();

    let (ticked, ()) = tokio::join!(
        service.handle_tick(cancellation_token.clone(), first_tick_context()),
        async {
            assert!(matches!(
                subscription
                    .wait_until_present(&CancellationToken::new())
                    .await,
                SyncHolderPresence::Present
            ));
            cancellation_token.cancel();
        },
    );

    ticked.expect("a poll never fails the service");

    let key_set = verification_key_set_holder
        .get()
        .expect("the usable keys of the set are held");
    let token = key.token(&key.header(), &json!({}));
    let CompactJwsParsing::Parsed(jws) = CompactJws::parse(&token) else {
        panic!("the token parses");
    };

    assert!(matches!(key_set.verify(&jws), JwsVerification::Verified(_)));

    server.stop().await;
}
