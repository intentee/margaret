use serde_json::json;

use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_jwks_endpoint::localhost_jwks_endpoint;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_holding::KeySetHolding;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;
use margaret_jws_verification_tests::fixture_rsa_key::FixtureRsaKey;

#[tokio::test(flavor = "multi_thread")]
async fn polls_the_key_set_of_a_jwks_endpoint() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(404, &json!({})),
        key_set: json_handler(
            200,
            &json!({ "keys": [FixtureRsaKey::load("rsa-kid").jwk()] }),
        ),
    })
    .await;

    assert!(matches!(
        localhost_jwks_endpoint()
            .first_poll(issuer.request_client())
            .await,
        KeySetRefresh::Refreshed(KeySetHolding::Held(_))
    ));

    issuer.stop().await;
}
