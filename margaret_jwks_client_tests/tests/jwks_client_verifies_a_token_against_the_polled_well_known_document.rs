use std::sync::Arc;
use std::time::Duration;

use reqwest::Client;
use reqwest::redirect::Policy;
use rustls::ClientConfig;
use tokio::task::yield_now;
use tokio_util::sync::CancellationToken;
use trzcina::ServiceBundle as _;
use url::Url;

use margaret_http_tests::mtls_fixture::MtlsFixture;
use margaret_jwks_client::JwksClient;
use margaret_jwks_client::public_token_verification::PublicTokenVerification;
use margaret_jwks_client_tests::running_jwks_server::RunningJwksServer;
use margaret_jwks_client_tests::test_claims::TestClaims;
use margaret_jwks_client_tests::test_instant::test_instant;
use margaret_jwks_endpoint::static_endpoint::StaticEndpoint;
use margaret_jwks_keygen::signs_claims::SignsClaims as _;
use margaret_jwks_roller::memory_jwks_secret_storage::MemoryJwksSecretStorage;
use margaret_jwks_roller::well_known_jwks_path::WELL_KNOWN_JWKS_PATH;
use margaret_jwks_roller_server::JwksRollerServerBundle;
use margaret_jwks_roller_server::JwksRollerServerBundleParams;

#[tokio::test(flavor = "multi_thread")]
async fn jwks_client_verifies_a_token_against_the_polled_well_known_document() {
    let fixture = MtlsFixture::new();

    let server_bundle = JwksRollerServerBundle::new(JwksRollerServerBundleParams {
        storage: Arc::new(MemoryJwksSecretStorage),
    });
    let public_jwks_handler = server_bundle.public_jwks_handler();
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

    let client_builder = Client::builder()
        .use_preconfigured_tls(ClientConfig::clone(&fixture.client_config))
        .https_only(true)
        .redirect(Policy::none())
        .timeout(Duration::from_secs(60));
    let jwks_url = Url::parse(&format!(
        "https://{}:{}{}",
        fixture.server_name,
        jwks_server.port(),
        WELL_KNOWN_JWKS_PATH,
    ))
    .expect("the jwks url parses");
    let jwks_client = JwksClient::create(Arc::new(StaticEndpoint::new(jwks_url)));
    let verifier = jwks_client.verifier();

    let poll_token = cancellation_token.clone();
    let poll_task = tokio::spawn(async move {
        jwks_client
            .run_with_client_builder(client_builder, poll_token)
            .await
    });

    let verified = loop {
        if let Ok(PublicTokenVerification::Verified(verified)) =
            verifier.verify::<TestClaims>(&token, test_instant(1_700_000_000))
        {
            break verified;
        }

        yield_now().await;
    };

    assert_eq!(verified, claims);

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
