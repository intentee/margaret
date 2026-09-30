use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn narrows_the_scope_of_a_refreshed_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let refresh_token = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let narrowed = refreshed_tokens(&fixture, &refresh_token, Some("profile")).await;

    assert_eq!(narrowed.status, 200);
    assert_eq!(narrowed.body["scope"], "profile");

    fixture.stop().await;
}
