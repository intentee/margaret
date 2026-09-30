use chrono::Utc;
use uuid::Uuid;

use margaret_oidc_provider::authenticated_end_user::AuthenticatedEndUser;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;

use crate::provider_fixture::ProviderFixture;
use crate::requested_consent::requested_consent;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_parameters::spa_parameters;

#[tokio::test]
async fn forgets_a_consent_decided_by_another_end_user() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let consent = requested_consent(&fixture, &spa_parameters()).await;
    let intruder = AuthenticatedEndUser {
        authenticated_at: Utc::now(),
        subject: Uuid::from_u128(99),
    };

    assert!(matches!(
        fixture
            .consent
            .decide(consent.id, &intruder, ConsentDecision::Approved)
            .await
            .expect("the consent reaches its state"),
        ConsentOutcome::Unknown
    ));
    assert!(matches!(
        fixture
            .consent
            .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
            .await
            .expect("the consent reaches its state"),
        ConsentOutcome::Unknown
    ));

    fixture.stop().await;
}
