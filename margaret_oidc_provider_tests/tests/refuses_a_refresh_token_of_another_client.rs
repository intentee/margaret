use serde_json::json;

use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_client::portal_client;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_oidc_provider_tests::service_client::service_client;
use margaret_oidc_provider_tests::service_secret::SERVICE_SECRET;

#[tokio::test]
async fn refuses_a_refresh_token_of_another_client() {
    let mut service = service_client();

    service.authorization_code = portal_client().authorization_code;

    let fixture = ProviderFixture::serving(vec![portal_client(), service], Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let foreign = fixture
        .post_form(
            "/token",
            &ClientCredentials::Basic {
                client_id: "service",
                secret: SERVICE_SECRET,
            },
            &json!({"grant_type": "refresh_token", "refresh_token": refresh_token}),
        )
        .await;

    assert_eq!(foreign.status, 400);
    assert_eq!(foreign.body["error"], "invalid_grant");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
