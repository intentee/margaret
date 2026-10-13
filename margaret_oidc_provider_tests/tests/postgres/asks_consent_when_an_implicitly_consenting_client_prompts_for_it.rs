use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider::consent_request::ConsentRequest;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::with_parameter::with_parameter;

#[tokio::test]
async fn asks_consent_when_an_implicitly_consenting_client_prompts_for_it() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::ConsentRequired(ConsentRequest { client_id, .. }) = fixture
        .authorized(&with_parameter(portal_parameters(), "prompt", "consent"))
        .await
    else {
        panic!("the end user is asked for consent");
    };

    assert_eq!(client_id, "portal");

    fixture.stop().await;
}
