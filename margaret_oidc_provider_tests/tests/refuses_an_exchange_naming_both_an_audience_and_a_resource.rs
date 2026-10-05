use serde_json::json;

use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_exchange_naming_both_an_audience_and_a_resource() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture.post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": "urn:ietf:params:oauth:grant-type:token-exchange", "subject_token": "a.b.c", "subject_token_type": "urn:ietf:params:oauth:token-type:id_token", "audience": "artifacts", "resource": "artifacts"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}
