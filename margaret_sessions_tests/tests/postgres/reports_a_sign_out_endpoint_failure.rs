use std::sync::Arc;

use cookie::Cookie;
use http::Method;

use margaret_database_tests::started_database::StartedDatabase;
use margaret_handler_error::handler_error::HandlerError;
use margaret_http::head_handler::HeadHandler;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::session_sign_out_endpoint::SessionSignOutEndpoint;
use margaret_sessions::sessions_error::SessionsError;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;

#[tokio::test]
async fn reports_a_sign_out_endpoint_failure() {
    let started = StartedDatabase::start().await;

    assert!(matches!(
        SessionSignOutEndpoint::create(
            Arc::new(IssuedSessions::host_only(
                started.database.clone(),
                session_store(),
                FIXTURE_AUDIENCE,
            )),
            "https://blog.example/welcome".to_string(),
        )
        .handle(&presenting_cookies(
            Method::POST,
            &[Cookie::new("__Host-margaret-session", "contract-secret")],
            PeerIdentity::Anonymous,
        ))
        .await,
        Err(HandlerError::Consumer { source })
            if matches!(source.downcast_ref::<SessionsError>(), Some(SessionsError::ForgetSession(_)))
    ));
}
