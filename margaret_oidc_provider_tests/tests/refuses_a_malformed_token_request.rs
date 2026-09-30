use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_malformed_token_request() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &json!({"code": "abc"}))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");
    assert_eq!(answer.header("cache-control"), "no-store");

    fixture.stop().await;
}
