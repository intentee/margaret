use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn exchanges_a_code_without_openid_for_an_access_token_alone() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&with_parameter(portal_parameters(), "scope", "profile"))
        .await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let answer = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("id_token").is_none());
    assert_eq!(jws_claims(&access_token)["aud"], "artifacts");

    fixture.stop().await;
}
