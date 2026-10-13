use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::constants::STAFF_CREDENTIALS;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::staff_code_exchange::staff_code_exchange;
use margaret_oidc_provider_tests::staff_parameters::staff_parameters;

#[tokio::test]
async fn refuses_a_code_without_openid_of_a_client_of_no_resources_since_its_token_would_reach_no_audience()
 {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) =
        fixture.authorized(&staff_parameters("profile")).await
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

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
