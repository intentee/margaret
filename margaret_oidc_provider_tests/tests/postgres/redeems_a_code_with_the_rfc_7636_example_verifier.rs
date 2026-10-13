use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_callback::PORTAL_CALLBACK;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

const CODE_CHALLENGE: &str = include_str!("../../fixtures/rfc7636/appendix_b_code_challenge.txt");
const CODE_VERIFIER: &str = include_str!("../../fixtures/rfc7636/appendix_b_code_verifier.txt");

#[tokio::test]
async fn redeems_a_code_with_the_rfc_7636_example_verifier() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&with_parameter(
            portal_parameters(),
            "code_challenge",
            CODE_CHALLENGE,
        ))
        .await
    else {
        panic!("the portal is issued a code");
    };
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({
                "code": issued_code(&redirect),
                "code_verifier": CODE_VERIFIER,
                "grant_type": "authorization_code",
                "redirect_uri": PORTAL_CALLBACK,
            }),
        )
        .await;

    assert_eq!(answer.status, 200);

    fixture.stop().await;
}
