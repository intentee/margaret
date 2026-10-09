use std::sync::Arc;

use cookie::Cookie;

use margaret_database_tests::table_privilege::TablePrivilege;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_oidc_provider::consent_handler::ConsentHandler;
use margaret_oidc_provider_tests::form_post_request::form_post_request;
use margaret_oidc_provider_tests::owned_body::owned_body;
use margaret_oidc_provider_tests::provider_fixture::ProviderFixture;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sql_identifier::table_namespace::TableNamespace;

#[tokio::test]
async fn reports_a_consent_decision_whose_session_cannot_be_resolved() {
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

    let answered =
        ConsentHandler::create(Arc::clone(&fixture.consent), Arc::clone(&fixture.sessions))
            .handle(
                &form_post_request(&[Cookie::new("__Host-margaret-session", "contract-secret")]),
                owned_body("decision=approve&id=00000000-0000-0000-0000-000000000000".to_string()),
                BodyLimit::new(1_024),
            )
            .await;

    fixture.stop().await;

    assert!(matches!(
        answered,
        Err(HandlerError::Consumer { source })
            if matches!(source.downcast_ref(), Some(SessionsError::FindSession(_)))
    ));
}
