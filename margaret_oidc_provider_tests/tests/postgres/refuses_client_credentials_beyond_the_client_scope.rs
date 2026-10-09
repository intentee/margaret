use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_client_credentials_beyond_the_client_scope() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials", "scope": "openid"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_scope");

    fixture.stop().await;
}
