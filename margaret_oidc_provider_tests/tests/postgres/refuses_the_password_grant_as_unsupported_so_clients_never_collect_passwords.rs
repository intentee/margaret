use serde_json::json;

use margaret_oidc_provider_tests::constants::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;

#[tokio::test]
async fn refuses_the_password_grant_as_unsupported_so_clients_never_collect_passwords() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &json!({"grant_type": "password"}),
        )
        .await;

    assert_eq!(answer.status, 400);
    assert_eq!(answer.body["error"], "unsupported_grant_type");

    fixture.stop().await;
}
