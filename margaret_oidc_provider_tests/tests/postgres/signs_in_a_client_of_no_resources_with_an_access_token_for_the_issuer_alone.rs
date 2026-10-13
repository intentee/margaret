use margaret_jws_verification::compact_jws::CompactJws;
use margaret_jws_verification::compact_jws_parsing::CompactJwsParsing;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::jws_claims::jws_claims;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::staff_code_exchange::staff_code_exchange;
use margaret_oidc_provider_tests::staff_credentials::STAFF_CREDENTIALS;
use margaret_oidc_provider_tests::staff_parameters::staff_parameters;

#[tokio::test]
async fn signs_in_a_client_of_no_resources_with_an_access_token_for_the_issuer_alone() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) =
        fixture.authorized(&staff_parameters("openid")).await
    else {
        panic!("the staff client is issued a code");
    };
    let answer = fixture
        .post_form(
            "/token",
            &STAFF_CREDENTIALS,
            &staff_code_exchange(&issued_code(&redirect)),
        )
        .await;
    let CompactJwsParsing::Parsed(access_token) = CompactJws::parse(answer.member("access_token"))
    else {
        panic!("the access token is a compact jws");
    };

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("id_token").is_some());
    assert_eq!(jws_claims(&access_token)["aud"], "https://localhost");

    fixture.stop().await;
}
