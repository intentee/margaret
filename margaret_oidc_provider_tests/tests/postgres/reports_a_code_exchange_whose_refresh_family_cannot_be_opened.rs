use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::code_exchange::code_exchange;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_credentials::PORTAL_CREDENTIALS;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_code_exchange_whose_refresh_family_cannot_be_opened() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "refresh_families",
        )
        .await;

    let answer = fixture
        .post_form(
            "/token",
            &PORTAL_CREDENTIALS,
            &code_exchange(&issued_code(&redirect)),
        )
        .await;

    assert_eq!(answer.status, 500);

    fixture.stop().await;
}
