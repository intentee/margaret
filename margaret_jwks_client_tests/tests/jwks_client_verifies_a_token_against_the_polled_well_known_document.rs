use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use reqwest::redirect::Policy;
use rustls::ClientConfig;
use tokio::task::yield_now;
use tokio_util::sync::CancellationToken;
use trzcina::Service as _;
use url::Url;

use margaret_endpoint::static_endpoint::StaticEndpoint;
use margaret_http::handler::Handler;
use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_jwks_client::public_jwks_poll_service::PublicJwksPollService;
use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
use margaret_jwks_client_tests::running_jwks_server::RunningJwksServer;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_keygen::jwks_secret_holder::JwksSecretHolder;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::jwks_document_holder::JwksDocumentHolder;
use margaret_jwks_roller_server::jwks_roller_service::JwksRollerService;
use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;

#[tokio::test(flavor = "multi_thread")]
async fn jwks_client_verifies_a_token_against_the_polled_well_known_document() {
    let fixture = MtlsFixture::new();

    let jwks_document_holder = JwksDocumentHolder::default();
    let jwks_secret_holder = JwksSecretHolder::default();
    let roll_service = JwksRollerService::new(
        jwks_document_holder.clone(),
        jwks_secret_holder.clone(),
        Arc::new(MemoryJwksSecretStorage),
    );
    let public_jwks_handler: Arc<dyn Handler> =
        Arc::new(PublicJwksHandler::new(jwks_document_holder));

    let cancellation_token = CancellationToken::new();
    let roll_token = cancellation_token.clone();
    let roll_task = tokio::spawn(async move { Box::new(roll_service).run(roll_token).await });

    let mut secret_subscription = jwks_secret_holder.subscribe();

    while secret_subscription.read_current().is_none() {
        secret_subscription.changed().await;
    }

    let secret = jwks_secret_holder.get().expect("the first roll published");
    let claims = TestClaims {
        exp: 1_700_000_060,
        sub: "subject".to_string(),
    };
    let token = secret
        .current
        .signing
        .sign(&claims)
        .await
        .expect("the claims sign");

    let jwks_server =
        RunningJwksServer::start(fixture.server_config.clone(), public_jwks_handler).await;

    let http_client = Client::builder()
        .use_preconfigured_tls(ClientConfig::clone(&fixture.client_config))
        .https_only(true)
        .redirect(Policy::none())
        .timeout(Duration::from_secs(60))
        .build()
        .expect("the test jwks http client builds");
    let jwks_url = Url::parse(&format!(
        "https://{}:{}{}",
        fixture.server_name,
        jwks_server.port(),
        WELL_KNOWN_JWKS_PATH,
    ))
    .expect("the jwks url parses");

    let verifier = PublicJwksVerifier::new();

    assert!(!verifier.is_ready());

    let poll_service = PublicJwksPollService::new(
        verifier.public_jwks_holder(),
        Arc::new(StaticEndpoint::new(jwks_url)),
        http_client,
    );
    let poll_token = cancellation_token.clone();
    let poll_task = tokio::spawn(async move { Box::new(poll_service).run(poll_token).await });

    let verified = loop {
        if let Ok(verified) = verifier.verify::<TestClaims>(&token, test_instant(1_700_000_000)) {
            break verified;
        }

        yield_now().await;
    };

    assert_eq!(verified, claims);
    assert!(verifier.is_ready());

    cancellation_token.cancel();
    roll_task
        .await
        .expect("the roll task joins")
        .expect("the roll service shuts down cleanly");
    poll_task
        .await
        .expect("the poll task joins")
        .expect("the poll service shuts down cleanly");
    jwks_server.stop().await;
}
