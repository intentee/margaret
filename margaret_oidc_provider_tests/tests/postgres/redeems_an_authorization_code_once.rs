use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn redeems_an_authorization_code_once() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let code = issued_code(&redirect);
    let first = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;
    let replayed = fixture
        .post_form("/token", &PORTAL_CREDENTIALS, &code_exchange(&code))
        .await;
    let refreshed = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({
                "grant_type": "refresh_token",
                "refresh_token": first.body["refresh_token"],
            }),
        )
        .await;

    assert_eq!(first.status, 200);
    assert_eq!(replayed.status, 400);
    assert_eq!(replayed.body["error"], "invalid_grant");
    assert_eq!(refreshed.body["error"], "invalid_grant");

    fixture.stop().await;
}
