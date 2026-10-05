use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::pkce_verifier::PKCE_VERIFIER;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_callback::SPA_CALLBACK;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn refuses_a_code_exchange_that_names_no_resource_of_several() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(consent) =
        fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };
    let ConsentOutcome::Redirected(redirect) = fixture
        .consent
        .decide(consent.id, &signed_in_end_user(), ConsentDecision::Approved)
        .await
        .expect("the consent reaches its state")
    else {
        panic!("the approved consent redirects");
    };
    let code = issued_code(&redirect);
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Absent,
            &json!({
                "client_id": "spa",
                "code": code,
                "code_verifier": PKCE_VERIFIER,
                "grant_type": "authorization_code",
                "redirect_uri": SPA_CALLBACK,
            }),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
