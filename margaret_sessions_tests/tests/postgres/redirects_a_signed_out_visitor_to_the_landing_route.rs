use std::sync::Arc;

use cookie::Cookie;
use http::Method;
use http::header::LOCATION;
use http::header::SET_COOKIE;

use margaret_http::head_handler::HeadHandler;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_peer_identity::peer_identity::PeerIdentity;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::session_sign_out_endpoint::SessionSignOutEndpoint;
use margaret_sessions_tests::presenting_cookies::presenting_cookies;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn redirects_a_signed_out_visitor_to_the_landing_route() {
    let started = started_with_sessions().await;
    let signed_out = SessionSignOutEndpoint::create(
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
    .await;

    assert!(matches!(
        signed_out,
        Ok(ResponseContinuation::Done(response))
            if response.status() == 303
                && response.header_value(&LOCATION) == Some("https://blog.example/welcome")
                && response
                    .headers()
                    .iter()
                    .filter(|header| header.name == SET_COOKIE.as_str())
                    .count()
                    == 2
    ));
}
