use serde_json::json;

use margaret_oidc_provider_tests::client_credentials::ClientCredentials;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_an_introspection_by_an_unauthenticated_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/introspect",
            &ClientCredentials::Absent,
            &json!({"token": "any"}),
        )
        .await;

    assert_eq!(answer.status, 401);

    fixture.stop().await;
}
