use cookie::Cookie;

use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_oidc_provider_tests::authorization_page_request::authorization_page_request;
use margaret_oidc_provider_tests::fixture_authorization_handler::fixture_authorization_handler;
use margaret_oidc_provider_tests::fixture_consent_view::FixtureConsentView;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_oidc_provider_tests::spa_parameters::spa_parameters;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_an_authorization_page_whose_session_cannot_be_resolved() {
    let fixture = ProviderFixture::start(Vec::new()).await;

    fixture
        .storage
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Framework,
            "sessions",
        )
        .await;

    let answered = fixture_authorization_handler(&fixture, FixtureConsentView)
        .handle(&authorization_page_request(
            &spa_parameters(),
            &[Cookie::new("__Host-margaret-session", "contract-secret")],
        ))
        .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Err(HandlerError::Consumer { source })
            if matches!(source.downcast_ref(), Some(SessionsError::FindSession(_)))
    ));
}
