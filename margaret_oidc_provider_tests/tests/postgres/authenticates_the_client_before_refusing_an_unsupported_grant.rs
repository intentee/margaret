use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn authenticates_the_client_before_refusing_an_unsupported_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Absent,
            &json!({"grant_type": "password"}),
        )
        .await;

    assert_eq!(answer.status, 401);
    assert_eq!(answer.body["error"], "invalid_client");

    fixture.stop().await;
}
