use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_client_with_a_wrong_secret_with_a_basic_challenge() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Basic {
                client_id: "portal",
                secret: "wrong",
            },
            &json!({"grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 401);
    assert_eq!(answer.body["error"], "invalid_client");
    assert_eq!(answer.header("www-authenticate"), "Basic");

    fixture.stop().await;
}
