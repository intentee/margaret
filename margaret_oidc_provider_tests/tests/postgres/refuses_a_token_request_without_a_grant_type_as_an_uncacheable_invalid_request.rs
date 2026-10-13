use serde_json::json;

use margaret_oidc_provider_tests::constants::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_token_request_without_a_grant_type_as_an_uncacheable_invalid_request() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &json!({"code": "abc"}))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");
    assert_eq!(answer.header("cache-control"), "no-store");

    fixture.stop().await;
}
