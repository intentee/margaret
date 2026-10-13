use serde_json::json;

use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::serving_unreachable_assertions::serving_unreachable_assertions;

#[tokio::test]
async fn reports_an_introspection_whose_client_assertion_cannot_be_remembered() {
    let fixture = serving_unreachable_assertions().await;
    let answer = fixture
        .post_form(
            "/introspect",
            &SERVICE_CREDENTIALS,
            &json!({"token": "an-access-token"}),
        )
        .await;

    assert_eq!(answer.status, 500);

    fixture.stop().await;
}
