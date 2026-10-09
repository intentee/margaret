use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::grant_interference::GrantInterference;
use margaret_oidc_provider_tests::grant_operation::GrantOperation;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn reports_a_pending_consent_that_cannot_be_taken() {
    let fixture = ProviderFixture::interfered(GrantInterference::Failing(
        GrantOperation::TakePendingAuthorization,
    ))
    .await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };

    assert!(matches!(
        fixture
            .consent
            .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
            .await,
        Err(ProviderError::TakePendingAuthorization(_))
    ));

    fixture.stop().await;
}
