use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_a_code_exchanged_by_a_client_without_the_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let answer = fixture
        .post_form("/token", &SERVICE_CREDENTIALS, &code_exchange(&code))
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
