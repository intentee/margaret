use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::with_parameter::with_parameter;
use oauth2::PkceCodeChallenge;
use oauth2::PkceCodeVerifier;

const RESERVED_VERIFIER: &str = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk/";

#[tokio::test]
async fn refuses_a_code_presented_with_a_verifier_of_reserved_characters() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let challenge = PkceCodeChallenge::from_code_verifier_sha256(&PkceCodeVerifier::new(
        RESERVED_VERIFIER.to_string(),
    ));
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&with_parameter(
            portal_parameters(),
            "code_challenge",
            challenge.as_str(),
        ))
        .await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &with_parameter(code_exchange(&code), "code_verifier", RESERVED_VERIFIER),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");

    fixture.stop().await;
}
