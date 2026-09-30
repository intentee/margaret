use margaret_oidc_provider::consent_request::ConsentRequest;

use crate::fixture_scopes::fixture_scopes;
use crate::provider_fixture::ProviderFixture;
use crate::requested_consent::requested_consent;
use crate::spa_parameters::spa_parameters;

#[tokio::test]
async fn asks_consent_of_a_prompting_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let ConsentRequest {
        client_id, scopes, ..
    } = requested_consent(&fixture, &spa_parameters()).await;

    assert_eq!(client_id.as_str(), "spa");
    assert_eq!(scopes, fixture_scopes(&["openid"]));

    fixture.stop().await;
}
