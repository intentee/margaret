use serde_json::json;

use crate::provider_fixture::ProviderFixture;
use crate::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_an_introspection_by_a_client_not_permitted_to_introspect() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/introspect",
            &SERVICE_CREDENTIALS,
            &json!({"token": "any"}),
        )
        .await;

    assert_eq!(answer.status, 403);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
