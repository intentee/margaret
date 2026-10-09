use serde_json::json;

use margaret_issuer_directory_tests::fixture_issuer_routes::FixtureIssuerRoutes;
use margaret_issuer_directory_tests::json_handler::json_handler;
use margaret_issuer_directory_tests::localhost_oidc_issuer::localhost_oidc_issuer;
use margaret_issuer_directory_tests::running_fixture_issuer::RunningFixtureIssuer;
use margaret_issuer_key_set::key_set_refresh::KeySetRefresh;

#[tokio::test(flavor = "multi_thread")]
async fn keeps_awaiting_metadata_of_another_issuer() {
    let issuer = RunningFixtureIssuer::start(FixtureIssuerRoutes {
        discovery: json_handler(
            200,
            &json!({ "issuer": "https://attacker.example", "jwks_uri": "https://localhost/jwks" }),
        ),
        key_set: json_handler(200, &json!({ "keys": [] })),
    })
    .await;

    assert!(matches!(
        localhost_oidc_issuer()
            .first_poll(issuer.request_client())
            .await,
        KeySetRefresh::Unchanged
    ));

    issuer.stop().await;
}
