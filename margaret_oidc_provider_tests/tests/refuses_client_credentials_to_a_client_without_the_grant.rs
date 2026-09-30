use serde_json::json;

use crate::client_credentials::ClientCredentials;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_client_credentials_to_a_client_without_the_grant() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Absent,
            &json!({"client_id": "spa", "grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
