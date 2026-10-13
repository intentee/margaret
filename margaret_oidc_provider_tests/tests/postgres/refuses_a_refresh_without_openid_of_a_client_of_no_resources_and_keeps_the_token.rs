use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::constants::STAFF_CREDENTIALS;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::staff_code_exchange::staff_code_exchange;
use margaret_oidc_provider_tests::staff_parameters::staff_parameters;

#[tokio::test]
async fn refuses_a_refresh_without_openid_of_a_client_of_no_resources_and_keeps_the_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture
        .authorized(&staff_parameters("openid profile"))
        .await
    else {
        panic!("the staff client is issued a code");
    };
    let refresh_token = fixture
        .post_form(
            "/token",
            &STAFF_CREDENTIALS,
            &staff_code_exchange(&issued_code(&redirect)),
        )
        .await
        .body["refresh_token"]
        .clone();
    let refused = fixture
        .post_form(
            "/token",
            &STAFF_CREDENTIALS,
            &json!({"grant_type": "refresh_token", "refresh_token": refresh_token, "scope": "profile"}),
        )
        .await;
    let refreshed = fixture
        .post_form(
            "/token",
            &STAFF_CREDENTIALS,
            &json!({"grant_type": "refresh_token", "refresh_token": refresh_token}),
        )
        .await;

    assert_eq!(refused.status, 400);
    assert_eq!(refused.body["error"], "invalid_target");
    assert_eq!(refreshed.status, 200);

    fixture.stop().await;
}
