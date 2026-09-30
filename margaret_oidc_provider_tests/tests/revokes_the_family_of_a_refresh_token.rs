use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn revokes_the_family_of_a_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": refresh_token, "token_type_hint": "refresh_token"}),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.header("cache-control"), "no-store");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None).await.body["error"],
        "invalid_grant"
    );

    fixture.stop().await;
}
