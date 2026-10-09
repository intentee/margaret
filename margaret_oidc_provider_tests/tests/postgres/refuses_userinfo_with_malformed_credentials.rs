use margaret_oidc_provider_tests::answer::Answer;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::provider_url::provider_url;

#[tokio::test]
async fn refuses_userinfo_with_malformed_credentials() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = Answer::of(
        fixture
            .client
            .get(provider_url("/userinfo"))
            .header("authorization", "Bearer")
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 400);

    fixture.stop().await;
}
