use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::grant_interference::GrantInterference;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn refuses_a_refresh_token_rotated_concurrently() {
    let fixture =
        ProviderFixture::interfered(GrantInterference::RotationBeforeRefreshRotates).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let raced = refreshed_tokens(&fixture, &refresh_token, None).await;

    assert_eq!(raced.status, 400);
    assert_eq!(
        raced.body["error_description"],
        "the refresh token was already rotated, so its family is revoked"
    );
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None).await.body["error_description"],
        "the refresh token is not known"
    );

    fixture.stop().await;
}
