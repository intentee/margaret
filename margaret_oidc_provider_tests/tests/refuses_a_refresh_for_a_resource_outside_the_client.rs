use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_refresh_for_a_resource_outside_the_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let answer = fixture.post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": "refresh_token", "refresh_token": refresh_token, "resource": "https://elsewhere.example"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
