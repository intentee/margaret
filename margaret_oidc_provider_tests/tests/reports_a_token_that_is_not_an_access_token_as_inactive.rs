use serde_json::json;

use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn reports_a_token_that_is_not_an_access_token_as_inactive() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let answer = fixture
        .post_form(
            "/introspect",
            &PORTAL_CREDENTIALS,
            &json!({"token": refresh_token}),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.body, json!({"active": false}));

    fixture.stop().await;
}
