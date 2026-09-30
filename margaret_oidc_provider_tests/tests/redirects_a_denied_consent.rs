use margaret_oidc_provider::consent_decision::ConsentDecision;

use crate::provider_fixture::ProviderFixture;
use crate::redirection::Redirection;
use crate::requested_consent::requested_consent;
use crate::signed_in_end_user::signed_in_end_user;
use crate::spa_parameters::spa_parameters;

#[tokio::test]
async fn redirects_a_denied_consent() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let consent = requested_consent(&fixture, &spa_parameters()).await;
    let outcome = fixture
        .consent
        .decide(consent.id, &signed_in_end_user(), ConsentDecision::Denied)
        .await
        .expect("the consent reaches its state");

    assert_eq!(
        Redirection::of_consent(&outcome).parameter("error"),
        "access_denied"
    );

    fixture.stop().await;
}
