use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::answer::Answer;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::provider_url::provider_url;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_userinfo_for_a_token_without_openid() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&with_parameter(portal_parameters(), "scope", "profile"))
        .await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let access_token = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
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
