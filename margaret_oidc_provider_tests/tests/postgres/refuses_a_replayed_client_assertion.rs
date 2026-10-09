use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_replayed_client_assertion() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let credentials = ClientCredentials::Assertion(fixture.assertion("service"));
    let form = json!({"grant_type": "client_credentials"});
    let spent = fixture.post_form("/token", &credentials, &form).await;
    let replayed = fixture.post_form("/token", &credentials, &form).await;

    assert_eq!(spent.status, 200);
    assert_eq!(replayed.status, 401);
    assert_eq!(replayed.body["error"], "invalid_client");

    fixture.stop().await;
}
