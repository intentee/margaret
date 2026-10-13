use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::constants::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn refuses_to_revoke_a_superseded_refresh_token_of_another_client_keeping_its_family() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let superseded =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let current = refreshed_tokens(&fixture, &superseded, None).await.body["refresh_token"].clone();
    let answer = fixture
        .post_form(
            "/revoke",
            &SERVICE_CREDENTIALS,
            &json!({"token": superseded}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_grant");
    assert_eq!(refreshed_tokens(&fixture, &current, None).await.status, 200);

    fixture.stop().await;
}
