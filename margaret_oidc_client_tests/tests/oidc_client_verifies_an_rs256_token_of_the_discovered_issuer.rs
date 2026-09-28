use std::sync::Arc;

use serde_json::Value;
use serde_json::json;
use tokio_util::sync::CancellationToken;

use margaret_http::request_authorization::RequestAuthorization;
use margaret_http_tests::static_handler::StaticHandler;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;
use margaret_oidc_client::oidc_client::OidcClient;
use margaret_oidc_client::oidc_token_verification::OidcTokenVerification;
use margaret_oidc_client::presented_bearer::PresentedBearer;
use margaret_oidc_client_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_oidc_client_tests::localhost_trust::localhost_trust;
use margaret_oidc_client_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_oidc_client_tests::signed_id_token::SignedIdToken;
use margaret_sync_holder::sync_holder_presence::SyncHolderPresence;

#[tokio::test(flavor = "multi_thread")]
async fn oidc_client_verifies_an_rs256_token_of_the_discovered_issuer() {
    let key = FixtureRsaKey::load("github-kid");
    let trust = localhost_trust();
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: Arc::new(StaticHandler {
            body: br#"{"issuer":"https://localhost","jwks_uri":"https://localhost/jwks"}"#.to_vec(),
            content_type: "application/json",
            status: 200,
        }),
        key_set: Arc::new(StaticHandler {
            body: json!({ "keys": [key.jwk()] }).to_string().into_bytes(),
            content_type: "application/json",
            status: 200,
        }),
    })
    .await;
    let client = OidcClient::create(Arc::new(trust.clone()));
    let verifier = client.verifier();
    let mut subscription = client.subscribe();
    let cancellation_token = CancellationToken::new();
    let poll_token = cancellation_token.clone();
    let client_builder = issuer.client_builder();
    let poll_task = tokio::spawn(async move {
        client
            .run_with_client_builder(client_builder, poll_token)
            .await
    });

    assert_eq!(
        subscription.wait_until_present(&cancellation_token).await,
        SyncHolderPresence::Present
    );

    let token = SignedIdToken {
        audience: json!("margaret"),
        exp: 9_999_999_999,
        typ: "JWT",
    }
    .signed_by(&key, &trust);
    let authorization = RequestAuthorization::parse(Some(&format!("Bearer {token}")));
    let presented =
        PresentedBearer::read(&authorization).expect("the system clock reads as a numeric date");
    let OidcTokenVerification::Verified(identity) = verifier.verify::<Value>(&presented) else {
        panic!("the token of the discovered issuer verifies");
    };

    assert_eq!(identity.claims["repository_id"], "74");

    cancellation_token.cancel();
    poll_task
        .await
        .expect("the poll task joins")
        .expect("the oidc client shuts down cleanly");
    issuer.stop().await;
}
