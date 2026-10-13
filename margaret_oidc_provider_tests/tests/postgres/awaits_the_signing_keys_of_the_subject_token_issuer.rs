use margaret_oidc_provider_tests::ci_issuer::CiIssuer;
use margaret_oidc_provider_tests::exchange_request::exchange_request;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn awaits_the_signing_keys_of_the_subject_token_issuer() {
    let ci = CiIssuer::awaiting_keys();
    let fixture = ProviderFixture::start(vec![ci.exchanger.clone()]).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &exchange_request(&ci.token("intentee/margaret")),
        )
        .await;

    assert_eq!(answer.status, 503);
    assert_eq!(answer.body["error"], "temporarily_unavailable");

    fixture.stop().await;
}
