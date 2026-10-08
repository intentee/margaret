use serde_json::json;

use margaret_oidc_provider_tests::grant_interference::GrantInterference;
use margaret_oidc_provider_tests::grant_operation::GrantOperation;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn reports_a_revocation_whose_refresh_token_cannot_be_found() {
    let fixture =
        ProviderFixture::interfered(GrantInterference::Failing(GrantOperation::FindRefreshToken))
            .await;
    let answer = fixture
        .post_form(
            "/revoke",
            &PORTAL_CREDENTIALS,
            &json!({"token": "a-refresh-token"}),
        )
        .await;

    assert_eq!(answer.status, 500);

    fixture.stop().await;
}
