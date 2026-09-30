use serde_json::json;

use crate::client_credentials::ClientCredentials;
use crate::jwt_parts::JwtParts;
use crate::pkce_verifier::PKCE_VERIFIER;
use crate::provider_fixture::ProviderFixture;
use crate::spa_callback::SPA_CALLBACK;
use crate::spa_code::spa_code;

#[tokio::test]
async fn exchanges_a_code_of_a_public_client_for_a_named_resource() {
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
                "resource": "reports",
            }),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert!(answer.body.get("refresh_token").is_none());
    assert_eq!(
        JwtParts::of(&answer.body["access_token"]).payload["aud"],
        json!(["reports", "https://localhost"])
    );
    assert_eq!(
        JwtParts::of(&answer.body["id_token"]).header["alg"],
        "ES256"
    );

    fixture.stop().await;
}
