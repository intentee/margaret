use margaret_oidc_provider::consent_decision::ConsentDecision;

use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::requested_consent::requested_consent;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_parameters::spa_parameters;

pub async fn spa_code(fixture: &ProviderFixture) -> String {
    let consent = requested_consent(fixture, &spa_parameters()).await;

    Redirection::of_consent(
        &fixture
            .consent
            .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
            .await
            .expect("the consent reaches its state"),
    )
    .parameter("code")
    .to_string()
}
