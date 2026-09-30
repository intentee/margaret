use margaret_oauth_vocabulary::scope::Scope;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::jwt_parts::JwtParts;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;

#[tokio::test]
async fn exchanges_an_authorization_code_for_tokens() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = portal_tokens(&fixture).await;
    let access_token = JwtParts::of(&answer.body["access_token"]);
    let id_token = JwtParts::of(&answer.body["id_token"]);

    assert_eq!(answer.header("cache-control"), "no-store");
    assert_eq!(answer.header("pragma"), "no-cache");
    assert_eq!(answer.body["token_type"], "bearer");
    assert_eq!(answer.body["scope"], "openid profile");
    assert_eq!(
        answer.body["refresh_token"].as_str().map(str::len),
        Some(43)
    );
    assert_eq!(access_token.header["typ"], "at+jwt");
    assert_eq!(
        access_token.payload["aud"],
        serde_json::json!(["artifacts", "https://localhost"])
    );
    assert_eq!(access_token.payload["client_id"], "portal");
    assert_eq!(access_token.payload["scope"], "openid profile");
    assert_eq!(access_token.payload["sub"], END_USER_SUBJECT.to_string());
    assert_eq!(id_token.header["alg"], "RS256");
    assert_eq!(id_token.payload["aud"], "portal");
    assert_eq!(id_token.payload["azp"], "portal");
    assert_eq!(id_token.payload["nonce"], "n-0S6_WzA2Mj");
    assert_eq!(id_token.payload["sub"], END_USER_SUBJECT.to_string());
    assert!(id_token.payload["auth_time"].is_i64());
    assert!(
        "openid profile"
            .split(' ')
            .map(str::parse::<Scope>)
            .all(|scope| scope.is_ok())
    );

    fixture.stop().await;
}
