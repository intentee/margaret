use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::grant_interference::GrantInterference;
use margaret_oidc_provider_tests::grant_operation::GrantOperation;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn reports_a_replayed_code_whose_family_cannot_be_revoked() {
    let fixture = ProviderFixture::interfered(GrantInterference::Failing(
        GrantOperation::RevokeRefreshFamily,
    ))
    .await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);

    portal_tokens(&fixture, &code).await;

    assert_eq!(
        fixture
            .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
            .await
            .status,
        500
    );

    fixture.stop().await;
}
