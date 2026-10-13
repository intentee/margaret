use std::sync::Arc;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::session_refresh_endpoint::SessionRefreshEndpoint;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sessions_tests::fixture_workload::fixture_workload;
use margaret_sessions_tests::form_request::form_request;
use margaret_sessions_tests::session_store::session_store;

#[tokio::test]
async fn reports_a_session_refresh_in_a_database_without_its_tables() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        SessionRefreshEndpoint::create(Arc::new(IssuedSessions::host_only(
            started.database.clone(),
            session_store(),
            FIXTURE_AUDIENCE,
        )))
        .handle(
            &form_request(fixture_workload()),
            fixture_body(b"secret=contract"),
            BodyLimit::new(1024),
        )
        .await,
        Err(HandlerError::Consumer { source })
            if matches!(source.downcast_ref::<SessionsError>(), Some(SessionsError::FindSession(_)))
    ));
}
