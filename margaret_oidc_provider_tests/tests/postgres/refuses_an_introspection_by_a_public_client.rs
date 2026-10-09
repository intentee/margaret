use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_introspection_by_a_public_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/introspect",
            &ClientCredentials::Absent,
            &json!({"client_id": "spa", "token": "any"}),
        )
        .await;

    assert_eq!(answer.status, 403);
    assert_eq!(answer.body["error"], "unauthorized_client");

    fixture.stop().await;
}
