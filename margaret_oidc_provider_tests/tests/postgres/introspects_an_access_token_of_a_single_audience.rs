use serde_json::json;

use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn introspects_an_access_token_of_a_single_audience() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let access_token = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({"grant_type": "client_credentials", "scope": "profile"}),
        )
        .await
        .body["access_token"]
        .clone();
    let answer = fixture
        .post_form(
            "/introspect",
            &PORTAL_CREDENTIALS,
            &json!({"token": access_token}),
        )
        .await;

    assert_eq!(answer.body["active"], true);
    assert_eq!(answer.body["aud"], json!(["artifacts"]));
    assert_eq!(answer.body["sub"], "portal");

    fixture.stop().await;
}
