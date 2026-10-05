use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_to_revoke_a_refresh_token_of_another_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let answer = fixture
        .post_form(
            "/revoke",
            &SERVICE_CREDENTIALS,
            &json!({"token": refresh_token}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
