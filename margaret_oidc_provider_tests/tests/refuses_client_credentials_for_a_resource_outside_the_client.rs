use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_client_credentials_for_a_resource_outside_the_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials", "resource": ""}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
