use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn revokes_the_family_of_a_replayed_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let first =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();
    let rotated = refreshed_tokens(&fixture, &first, None).await.body["refresh_token"].clone();
    let replayed = refreshed_tokens(&fixture, &first, None).await;

    assert_eq!(replayed.status, 400);
    assert_eq!(replayed.body["error"], "invalid_grant");
    assert_eq!(
        refreshed_tokens(&fixture, &rotated, None).await.body["error"],
        "invalid_grant"
    );

    fixture.stop().await;
}
