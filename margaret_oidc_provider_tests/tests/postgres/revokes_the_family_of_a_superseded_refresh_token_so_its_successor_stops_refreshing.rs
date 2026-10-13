use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::constants::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn revokes_the_family_of_a_superseded_refresh_token_so_its_successor_stops_refreshing() {
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
            &PORTAL_CREDENTIALS,
            &json!({"token": superseded, "token_type_hint": "refresh_token"}),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(
        refreshed_tokens(&fixture, &current, None).await.body["error"],
        "invalid_grant"
    );

    fixture.stop().await;
}
