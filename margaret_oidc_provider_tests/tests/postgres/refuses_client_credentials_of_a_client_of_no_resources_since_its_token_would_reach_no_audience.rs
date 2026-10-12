use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::staff_credentials::STAFF_CREDENTIALS;

#[tokio::test]
async fn refuses_client_credentials_of_a_client_of_no_resources_since_its_token_would_reach_no_audience()
 {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &STAFF_CREDENTIALS,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
