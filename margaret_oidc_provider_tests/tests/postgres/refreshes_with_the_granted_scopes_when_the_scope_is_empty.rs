use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn refreshes_with_the_granted_scopes_when_the_scope_is_empty() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let refreshed = refreshed_tokens(&fixture, &refresh_token, Some("")).await;

    assert_eq!(refreshed.status, 200);
    assert_eq!(refreshed.body["scope"], "openid profile");

    fixture.stop().await;
}
