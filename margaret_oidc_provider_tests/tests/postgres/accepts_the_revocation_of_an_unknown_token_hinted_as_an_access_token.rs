use serde_json::json;

use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn accepts_the_revocation_of_an_unknown_token_hinted_as_an_access_token() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": "unknown", "token_type_hint": "access_token"}),
        )
        .await;

    assert_eq!(answer.status, 200);
    assert_eq!(answer.header("cache-control"), "no-store");

    fixture.stop().await;
}
