use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_request::ConsentRequest;
use margaret_oidc_provider_tests::fixture_scopes::fixture_scopes;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;

#[tokio::test]
async fn asks_consent_of_a_prompting_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(ConsentRequest {
        client_id, scopes, ..
    }) = fixture.authorized(&spa_parameters()).await
    else {
        panic!("the end user is asked for consent");
    };

    assert_eq!(client_id.as_str(), "spa");
    assert_eq!(scopes, fixture_scopes(&["openid"]));

    fixture.stop().await;
}
