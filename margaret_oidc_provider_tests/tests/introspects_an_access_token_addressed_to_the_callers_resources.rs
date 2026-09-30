use serde_json::json;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::portal_credentials::PORTAL_CREDENTIALS;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn introspects_an_access_token_addressed_to_the_callers_resources() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let access_token = portal_tokens(&fixture).await.body["access_token"].clone();
    let answer = fixture
        .post_form(
            "/introspect",
            &PORTAL_CREDENTIALS,
            &json!({"token": access_token}),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.header("cache-control"), "no-store");
    assert_eq!(answer.body["active"], true);
    assert_eq!(
        answer.body["aud"],
        json!(["artifacts", "https://localhost"])
    );
    assert_eq!(answer.body["client_id"], "portal");
    assert_eq!(answer.body["iss"], "https://localhost");
    assert_eq!(answer.body["scope"], "openid profile");
    assert_eq!(answer.body["sub"], END_USER_SUBJECT.to_string());
    assert_eq!(answer.body["token_type"], "bearer");
    assert!(answer.body["exp"].is_i64());
    assert!(answer.body["iat"].is_i64());

    fixture.stop().await;
}
