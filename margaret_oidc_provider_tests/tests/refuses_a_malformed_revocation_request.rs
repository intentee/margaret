use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_a_malformed_revocation_request() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token_type_hint": "refresh_token"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_request");

    fixture.stop().await;
}
