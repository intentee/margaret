use serde_json::json;

use crate::jwt_parts::JwtParts;
use crate::provider_fixture::ProviderFixture;
use crate::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn issues_client_credentials_tokens() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &SERVICE_CREDENTIALS,
            &json!({"grant_type": "client_credentials"}),
        )
        .await;
    let access_token = JwtParts::of(&answer.body["access_token"]);

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("refresh_token").is_none());
    assert_eq!(answer.body["scope"], "artifacts:read");
    assert_eq!(access_token.payload["aud"], "artifacts");
    assert_eq!(access_token.payload["client_id"], "service");
    assert_eq!(access_token.payload["sub"], "service");

    fixture.stop().await;
}
