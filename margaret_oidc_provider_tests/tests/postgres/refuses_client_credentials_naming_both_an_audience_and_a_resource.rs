use serde_json::json;

use margaret_oidc_provider_tests::constants::LEDGER_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_client_credentials_naming_both_an_audience_and_a_resource() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &LEDGER_CREDENTIALS,
            &json!({"audience": "reports", "grant_type": "client_credentials", "resource": "artifacts"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}
