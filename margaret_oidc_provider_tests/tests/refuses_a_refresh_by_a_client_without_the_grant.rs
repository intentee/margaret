use serde_json::json;

use crate::provider_fixture::ProviderFixture;
use crate::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_a_refresh_by_a_client_without_the_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "refresh_token", "refresh_token": "any"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
