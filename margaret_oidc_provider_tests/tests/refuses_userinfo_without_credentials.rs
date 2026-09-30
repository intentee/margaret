use crate::answer::Answer;
use crate::client_credentials::ClientCredentials;
use crate::provider_fixture::ProviderFixture;
use crate::provider_url::provider_url;

#[tokio::test]
async fn refuses_userinfo_without_credentials() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = Answer::of(
        ClientCredentials::Absent
            .presented_on(fixture.client.get(provider_url("/userinfo")))
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 401);
    assert_eq!(answer.header("www-authenticate"), "Bearer");

    fixture.stop().await;
}
