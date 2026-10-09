use chrono::Utc;
use cookie::SameSite;
use cookie::time::Duration;
use uuid::Uuid;

use margaret_identity_session::access_token_lifetime_secs::ACCESS_TOKEN_LIFETIME_SECS;
use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions::session_lifetime_secs::SESSION_LIFETIME_SECS;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn sets_host_only_session_cookies_a_browser_keeps_to_itself() {
    let started = started_with_sessions().await;
    let begun =
        IssuedSessions::host_only(started.database.clone(), session_store(), FIXTURE_AUDIENCE)
            .start(Uuid::new_v4(), Utc::now())
            .await
            .expect("the session starts");

    assert!(matches!(
        begun.cookie_changes.cookies.as_slice(),
        [secret, access]
            if secret.name() == "__Host-margaret-session"
                && secret.max_age() == Some(Duration::seconds(i64::from(SESSION_LIFETIME_SECS)))
                && access.name() == "__Host-margaret-session-access"
                && access.max_age() == Some(Duration::seconds(i64::from(ACCESS_TOKEN_LIFETIME_SECS)))
    ));
    assert!(begun.cookie_changes.cookies.iter().all(|cookie| {
        cookie.domain().is_none()
            && cookie.http_only() == Some(true)
            && cookie.path() == Some("/")
            && cookie.same_site() == Some(SameSite::Strict)
            && cookie.secure() == Some(true)
    }));
}
