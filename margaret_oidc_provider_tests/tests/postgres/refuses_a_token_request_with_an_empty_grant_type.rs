use serde_json::json;

use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_token_request_with_an_empty_grant_type() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": ""}))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}
