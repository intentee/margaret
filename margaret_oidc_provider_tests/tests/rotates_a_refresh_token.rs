use crate::end_user_subject::END_USER_SUBJECT;
use crate::jwt_parts::JwtParts;
use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn rotates_a_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let first = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let rotated = refreshed_tokens(&fixture, &first, None).await;

    assert_eq!(rotated.status, 200);
    assert_ne!(rotated.body["refresh_token"], first);
    assert_eq!(rotated.body["scope"], "openid profile");
    assert_eq!(
        JwtParts::of(&rotated.body["access_token"]).payload["sub"],
        END_USER_SUBJECT.to_string()
    );
    assert_eq!(
        refreshed_tokens(&fixture, &rotated.body["refresh_token"], None)
            .await
            .status,
        200
    );

    fixture.stop().await;
}
