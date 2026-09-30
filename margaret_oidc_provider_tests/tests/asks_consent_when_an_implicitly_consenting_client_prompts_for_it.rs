use margaret_oidc_provider::consent_request::ConsentRequest;

use crate::portal_parameters::portal_parameters;
use crate::provider_fixture::ProviderFixture;
use crate::requested_consent::requested_consent;
use crate::with_parameter::with_parameter;

#[tokio::test]
async fn asks_consent_when_an_implicitly_consenting_client_prompts_for_it() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let ConsentRequest { client_id, .. } = requested_consent(
        &fixture,
        &with_parameter(portal_parameters(), "prompt", "consent"),
    )
    .await;

    assert_eq!(client_id.as_str(), "portal");

    fixture.stop().await;
}
