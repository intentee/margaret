use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_pending_consent_that_cannot_be_taken() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Delete,
            TableNamespace::Framework,
            "pending_authorizations",
        )
        .await;

    assert!(matches!(
        fixture
            .consent
            .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
            .await,
        Err(ProviderError::AuthorizationGrants(
            AuthorizationGrantsError::TakePendingAuthorization(_)
        ))
    ));

    fixture.stop().await;
}
