use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_refresh_with_a_malformed_scope() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let answer = fixture.post_form("/token", &PORTAL_CREDENTIALS, &json!({"grant_type": "refresh_token", "refresh_token": refresh_token, "scope": "openid  profile"})).await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_scope");

    fixture.stop().await;
}
