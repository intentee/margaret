use std::sync::Arc;

use margaret_http::body_limit::BodyLimit;
use margaret_http::handles_limited_content::HandlesLimitedContent;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_http_tests::fixture_body::fixture_body;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::session_refresh_endpoint::SessionRefreshEndpoint;
use margaret_sessions_tests::form_request::form_request;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn refuses_a_session_refresh_from_an_unverified_peer() {
    let started = started_with_sessions().await;

    assert!(matches!(
        SessionRefreshEndpoint::create(Arc::new(IssuedSessions::host_only(
            started.database.clone(),
            session_store(),
            FIXTURE_AUDIENCE,
        )))
        .handle(
            &form_request(PeerIdentity::Anonymous),
            fixture_body(b"secret=anything"),
            BodyLimit::new(1024),
        )
        .await,
        Ok(ResponseContinuation::Done(response)) if response.status() == 403
    ));
}
