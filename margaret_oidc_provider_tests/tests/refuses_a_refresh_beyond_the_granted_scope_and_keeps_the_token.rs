use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_a_refresh_beyond_the_granted_scope_and_keeps_the_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&with_parameter(portal_parameters(), "scope", "openid"))
        .await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let refresh_token = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await
        .body["refresh_token"]
        .clone();
    let exceeding = refreshed_tokens(&fixture, &refresh_token, Some("openid profile")).await;

    assert_eq!(exceeding.status, 400);
    assert_eq!(exceeding.body["error"], "invalid_scope");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
