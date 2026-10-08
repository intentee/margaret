use chrono::Utc;
use uuid::Uuid;

use margaret_authorization_grants::pending_authorization::PendingAuthorization;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;
use margaret_oidc_provider_tests::fixture_authorization_grant::fixture_authorization_grant;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_callback::SPA_CALLBACK;
use margaret_registered_claims::numeric_date::NumericDate;

#[tokio::test]
async fn forgets_an_expired_pending_consent() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let id = Uuid::new_v4();

    fixture
        .clients
        .grants
        .hold_pending_authorization(
            id,
            PendingAuthorization {
                expires_at: NumericDate::from(Utc::now()),
                grant: fixture_authorization_grant("spa", SPA_CALLBACK),
                state: None,
            },
        )
        .await
        .expect("the fixture store holds the consent");

    assert!(matches!(
        fixture
            .consent
            .decide(id, &signed_in_end_user(), ConsentDecision::Approved)
            .await,
        Ok(ConsentOutcome::Unknown)
    ));

    fixture.stop().await;
}
