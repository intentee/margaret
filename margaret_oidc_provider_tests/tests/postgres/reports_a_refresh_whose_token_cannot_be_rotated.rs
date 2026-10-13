use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_oidc_provider::authorization_outcome::AuthorizationOutcome;
use margaret_oidc_provider_tests::issued_code::issued_code;
use margaret_oidc_provider_tests::portal_parameters::portal_parameters;
use margaret_oidc_provider_tests::portal_tokens::portal_tokens;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::refreshed_tokens::refreshed_tokens;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_refresh_whose_token_cannot_be_rotated() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let AuthorizationOutcome::Redirected(redirect) = fixture.authorized(&portal_parameters()).await
    else {
        panic!("the portal is issued a code");
    };
    let refresh_token =
        portal_tokens(&fixture, &issued_code(&redirect)).await.body["refresh_token"].clone();

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Update,
            TableNamespace::Framework,
            "refresh_tokens",
        )
        .await;

    assert_eq!(
        refreshed_tokens(&fixture, &refresh_token, None)
            .await
            .status,
        500
    );

    fixture.stop().await;
}
