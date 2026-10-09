use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider::end_user_authentication::EndUserAuthentication;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_oidc_provider_tests::validated_form::validated_form;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_pending_consent_that_cannot_be_held() {
    let fixture = ProviderFixture::start(Vec::new()).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "pending_authorizations",
        )
        .await;

    let outcome = fixture
        .authorization
        .authorize(
            validated_form(&spa_parameters()),
            &EndUserAuthentication::Authenticated(signed_in_end_user()),
        )
        .await;

    assert!(matches!(
        outcome,
        Err(ProviderError::AuthorizationGrants(
            AuthorizationGrantsError::HoldPendingAuthorization(_)
        ))
    ));

    fixture.stop().await;
}
