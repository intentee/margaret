use margaret_oidc_provider_tests::answer::Answer;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::provider_url::provider_url;

#[tokio::test]
async fn refuses_userinfo_with_a_rejected_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = Answer::of(
        ClientCredentials::Bearer("a.b.c".to_string())
            .presented_on(fixture.client.get(provider_url("/userinfo")))
            .send()
            .await
            .expect("the provider answers"),
    )
    .await;

    assert_eq!(answer.status, 401);
    assert_eq!(
        answer.header("www-authenticate"),
        r#"Bearer error="invalid_token""#
    );

    fixture.stop().await;
}
