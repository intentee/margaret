use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_client_without_credentials_without_a_challenge() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Absent,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 401);
    assert_eq!(answer.body["error"], "invalid_client");
    assert!(answer.headers.get("www-authenticate").is_none());

    fixture.stop().await;
}
