use serde_json::json;

use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::end_user_subject::END_USER_SUBJECT;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::provider_url::provider_url;

#[tokio::test]
async fn answers_userinfo_with_the_subject_of_the_access_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let access_token = portal_tokens(&fixture).await.body["access_token"]
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

    assert_eq!(answer.status, 200);
    assert_eq!(answer.header("cache-control"), "no-store");
    assert_eq!(
        answer.body,
        json!({"name": "Ada", "sub": END_USER_SUBJECT.to_string()})
    );

    fixture.stop().await;
}
