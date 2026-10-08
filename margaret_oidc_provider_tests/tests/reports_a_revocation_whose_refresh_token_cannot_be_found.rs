use serde_json::json;

use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_revocation_whose_refresh_token_cannot_be_found() {
    let fixture = ProviderFixture::start(Vec::new()).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "refresh_tokens",
        )
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
