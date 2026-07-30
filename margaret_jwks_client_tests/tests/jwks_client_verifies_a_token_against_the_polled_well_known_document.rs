use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use reqwest::redirect::Policy;
use rustls::ClientConfig;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_jwks_client::JwksClient;
use margaret_jwks_client::access_token_verification::AccessTokenVerification;
use margaret_jwks_client_tests::running_jwks_server::RunningJwksServer;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

#[tokio::test(flavor = "multi_thread")]
async fn jwks_client_verifies_a_token_against_the_polled_well_known_document() {
    let fixture = MtlsFixture::new();

    let server_bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(MemoryJwksSecretStorage),
    });
    let public_jwks_handler = server_bundle.public_jwks_handler();
    let jwks_document_holder = server_bundle.jwks_document_holder();
    let jwks_secret_holder = server_bundle.jwks_secret_holder();
    let mut roll_services = server_bundle
        .services()
        .await
        .expect("the server bundle exposes services");
    let roll_service = roll_services
        .pop()
        .expect("the server bundle exposes the roll service");

    let cancellation_token = CancellationToken::new();
    let roll_token = cancellation_token.clone();
    let roll_task = tokio::spawn(async move { roll_service.run(roll_token).await });

    let mut document_subscription = jwks_document_holder.subscribe();

    assert_eq!(
        document_subscription
            .wait_until_present(&cancellation_token)
            .await,
        SyncHolderPresence::Present
    );

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

    let client_builder = Client::builder()
        .use_preconfigured_tls(ClientConfig::clone(&fixture.client_config))
        .https_only(true)
        .redirect(Policy::none())
        .timeout(Duration::from_mins(1));
    let jwks_url = Url::parse(&format!(
        "https://{}:{}{}",
        fixture.server_name,
        jwks_server.port(),
        WELL_KNOWN_JWKS_PATH,
    ))
    .expect("the jwks url parses");
    let jwks_client = JwksClient::create(Arc::new(StaticEndpoint::new(jwks_url)));
    let verifier = jwks_client.verifier();
    let mut jwks_subscription = jwks_client.subscribe();

    let poll_token = cancellation_token.clone();
    let poll_task = tokio::spawn(async move {
        jwks_client
            .run_with_client_builder(client_builder, poll_token)
            .await
    });

    assert_eq!(
        jwks_subscription
            .wait_until_present(&cancellation_token)
            .await,
        SyncHolderPresence::Present
    );

    let verification = verifier
        .verify::<TestClaims>(&token, test_instant(1_700_000_000))
        .expect("the polled document verifies the token");

    let AccessTokenVerification::Verified(verification_outcome) = verification else {
        panic!("the polled document verifies the token");
    };

    assert_eq!(verification_outcome, claims);

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
