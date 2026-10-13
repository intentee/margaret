use serde_json::json;

use margaret_oidc_provider_tests::service_credentials::SERVICE_CREDENTIALS;
use margaret_oidc_provider_tests::serving_unreachable_assertions::serving_unreachable_assertions;

#[tokio::test]
async fn reports_a_client_assertion_that_cannot_be_remembered() {
    let fixture = serving_unreachable_assertions().await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;

    assert_eq!(answer.status, 500);

    fixture.stop().await;
}
