use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::grant_interference::GrantInterference;
use margaret_oidc_provider_tests::grant_operation::GrantOperation;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::validated_form::validated_form;

#[tokio::test]
async fn reports_a_code_that_cannot_be_issued() {
    let fixture =
        ProviderFixture::interfered(GrantInterference::Failing(GrantOperation::IssueCode)).await;
    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&portal_parameters()),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await;

    assert!(matches!(outcome, Err(ProviderError::IssueCode(_))));

    fixture.stop().await;
}
