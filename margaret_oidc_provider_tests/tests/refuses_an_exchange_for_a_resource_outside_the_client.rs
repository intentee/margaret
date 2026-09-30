use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_exchange_for_a_resource_outside_the_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture.post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": "urn:ietf:params:oauth:grant-type:token-exchange", "subject_token": "a.b.c", "subject_token_type": "urn:ietf:params:oauth:token-type:id_token", "audience": "https://elsewhere.example"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
