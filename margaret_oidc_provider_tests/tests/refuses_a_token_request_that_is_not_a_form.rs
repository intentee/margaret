use serde_json::json;

use margaret_oidc_provider_tests::answer::Answer;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::provider_url::provider_url;

#[tokio::test]
async fn refuses_a_token_request_that_is_not_a_form() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = Answer::of(
        PORTAL_CREDENTIALS
            .presented_on(fixture.client.post(provider_url("/token")))
            .json(&json!({"grant_type": "client_credentials"}))
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 415);

    fixture.stop().await;
}
