use crate::portal_tokens::portal_tokens;
use crate::provider_fixture::ProviderFixture;
use crate::refreshed_tokens::refreshed_tokens;

#[tokio::test]
async fn revokes_the_family_of_a_replayed_refresh_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let first = portal_tokens(&fixture).await.body["refresh_token"].clone();
    let rotated = refreshed_tokens(&fixture, &first, None).await.body["refresh_token"].clone();
    let replayed = refreshed_tokens(&fixture, &first, None).await;

    assert_eq!(replayed.status, 400);
    assert_eq!(replayed.body["error"], "invalid_grant");
    assert_eq!(
        refreshed_tokens(&fixture, &rotated, None).await.body["error"],
        "invalid_grant"
    );

    fixture.stop().await;
}
