use margaret_authorization_grants::authorization_grants_error::AuthorizationGrantsError;
use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_oidc_provider::provider_error::ProviderError;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::signed_in_session_cookies::signed_in_session_cookies;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_an_authorization_page_whose_pending_consent_cannot_be_held() {
    let fixture = ProviderFixture::start(Vec::new()).await;
    let cookies = signed_in_session_cookies(&fixture.sessions).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Insert,
            TableNamespace::Framework,
            "pending_authorizations",
        )
        .await;

    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(&authorization_page_request(&spa_parameters(), &cookies))
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Err(HandlerError::Consumer { source })
            if matches!(
                source.downcast_ref(),
                Some(ProviderError::AuthorizationGrants(
                    AuthorizationGrantsError::HoldPendingAuthorization(_)
                ))
            )
    ));
}
