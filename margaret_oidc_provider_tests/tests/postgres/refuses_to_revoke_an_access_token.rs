use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_to_revoke_an_access_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let access_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["access_token"].clone();
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": access_token, "token_type_hint": "refresh_token"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unsupported_token_type");

    fixture.stop().await;
}
