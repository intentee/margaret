use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn refuses_a_code_exchange_for_a_resource_outside_the_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &with_parameter(
                code_exchange(&code),
                "resource",
                "https://elsewhere.example",
            ),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
