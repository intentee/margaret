use serde_json::json;

use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;
use crate::provider_url::provider_url;

#[tokio::test]
async fn refuses_userinfo_for_a_token_of_a_client_itself() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let access_token = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({"grant_type": "client_credentials", "scope": "openid"}),
        )
        .await
        .body["access_token"]
        .as_str()
        .expect("the access token is a string")
        .to_string();
    let answer = Answer::of(
        ClientCredentials::Bearer(access_token)
            .presented_on(fixture.client.get(provider_url("/userinfo")))
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 401);

    fixture.stop().await;
}
