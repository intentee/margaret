use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn accepts_the_revocation_of_an_unknown_token_with_an_unrecognized_hint() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": "unknown", "token_type_hint": "device_code"}),
        )
        .await;

    assert_eq!(answer.status, 200);

    fixture.stop().await;
}
