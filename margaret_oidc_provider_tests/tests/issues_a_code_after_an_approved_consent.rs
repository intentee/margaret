use margaret_oidc_provider::consent_decision::ConsentDecision;

use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::requested_consent::requested_consent;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_parameters::spa_parameters;

#[tokio::test]
async fn issues_a_code_after_an_approved_consent() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let consent = requested_consent(&fixture, &spa_parameters()).await;
    let outcome = fixture
        .consent
        .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
        .await
        .expect("the consent reaches its state");
    let redirection = Redirection::of_consent(&outcome);

    assert_eq!(redirection.parameter("code").len(), 43);
    assert_eq!(redirection.parameter("iss"), "https://localhost");
    assert_eq!(redirection.parameter("state"), "af0ifjsldkj");

    fixture.stop().await;
}
