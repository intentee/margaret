use serde_json::json;

use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;
use crate::service_credentials::SERVICE_CREDENTIALS;

#[tokio::test]
async fn refuses_to_revoke_a_refresh_token_of_another_client() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let answer = fixture
        .post_form(
            "/revoke",
            &SERVICE_CREDENTIALS,
            &json!({"token": refresh_token}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unauthorized_client");
    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
