use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_to_revoke_an_access_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": "any", "token_type_hint": "access_token"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unsupported_token_type");

    fixture.stop().await;
}
