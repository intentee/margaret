use serde_json::json;

use margaret_jose_parameters::jws_algorithm::JwsAlgorithm;
use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_jws_verification::parameter_value::ParameterValue;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_decision::ConsentDecision;
use margaret_oidc_provider::consent_outcome::ConsentOutcome;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::pkce_verifier::PKCE_VERIFIER;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_end_user::signed_in_end_user;
use margaret_oidc_provider_tests::spa_callback::SPA_CALLBACK;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn exchanges_a_code_of_a_public_client_for_a_named_resource() {
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
                "resource": "reports",
            }),
        )
        .await;

    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };
    let CompactJwsParsing::Parsed(id_token) = CompactJws::parse(answer.member("id_token")) else {
        panic!("the id token is a compact jws");
    };

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("refresh_token").is_none());
    assert_eq!(
        jws_claims(&access_token)["aud"],
        json!(["reports", "https://localhost"])
    );
    assert_eq!(
        id_token.alg(),
        &ParameterValue::Supported(JwsAlgorithm::Es256)
    );

    fixture.stop().await;
}
