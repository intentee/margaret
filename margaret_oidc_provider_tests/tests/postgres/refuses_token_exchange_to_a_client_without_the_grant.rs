use serde_json::json;

use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_token_exchange_to_a_client_without_the_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture.post_form("/token", &SERVICE_CREDENTIALS, &json!({"grant_type": "urn:ietf:params:oauth:grant-type:token-exchange", "subject_token": "a.b.c", "subject_token_type": "urn:ietf:params:oauth:token-type:id_token"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
