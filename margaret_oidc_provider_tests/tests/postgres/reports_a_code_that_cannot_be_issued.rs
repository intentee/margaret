use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_code_that_cannot_be_issued() {
    let fixture = ProviderFixture::start(Vec::new()).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "authorization_codes",
        )
        .await;

    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&portal_parameters()),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await;

    assert!(matches!(
        outcome,
        Err(ProviderError::AuthorizationGrants(
            AuthorizationGrantsError::IssueCode(_)
        ))
    ));

    fixture.stop().await;
}
