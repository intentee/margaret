use serde_json::json;

use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_introspection_without_a_token_as_invalid_request_not_inactive() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form("/introspect", &PORTAL_CREDENTIALS, &json!({}))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}
