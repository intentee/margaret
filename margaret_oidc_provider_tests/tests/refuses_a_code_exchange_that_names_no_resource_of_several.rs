use serde_json::json;

use crate::client_credentials::ClientCredentials;
use crate::pkce_verifier::PKCE_VERIFIER;
use crate::provider_fixture::ProviderFixture;
use crate::spa_callback::SPA_CALLBACK;
use crate::spa_code::spa_code;

#[tokio::test]
async fn refuses_a_code_exchange_that_names_no_resource_of_several() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let code = spa_code(&fixture).await;
    let answer = fixture
        .post_form(
            "/token",
            &ClientCredentials::Absent,
            &json!({
                "client_id": "spa",
                "code": code,
                "code_verifier": PKCE_VERIFIER,
                "grant_type": "authorization_code",
                "redirect_uri": SPA_CALLBACK,
            }),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "invalid_target");

    fixture.stop().await;
}
