use margaret_oidc_provider_tests::answer::Answer;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::provider_url::provider_url;

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
