use margaret_http_tests::redirection::Redirection;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn redirects_a_denied_consent() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };
    let outcome = fixture
        .consent
        .decide(consent.id, &signed_in_end_user(), ConsentDecision::Denied)
        .await
        .expect("the consent reaches its state");
    let ConsentOutcome::Redirected(response) = outcome else {
        panic!("the consent redirects");
    };

    assert_eq!(
        Redirection::of(&response).parameter("error"),
        "access_denied"
    );

    fixture.stop().await;
}
