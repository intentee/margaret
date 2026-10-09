use chrono::Utc;
use cookie::Cookie;
use uuid::Uuid;

use margaret_jwt_verification_tests::fixture_audience::FIXTURE_AUDIENCE;
use margaret_sessions::cookie_domain::CookieDomain;
use margaret_sessions::issued_sessions::IssuedSessions;
use margaret_sessions_tests::session_store::session_store;
use margaret_sessions_tests::started_with_sessions::started_with_sessions;

#[tokio::test]
async fn shares_session_cookies_with_the_declared_domain() {
    let started = started_with_sessions().await;
    let begun = IssuedSessions::shared_with_domain(
        started.database.clone(),
        session_store(),
        FIXTURE_AUDIENCE,
        "intentee.ai"
            .parse::<CookieDomain>()
            .expect("the domain is read"),
    )
    .start(Uuid::new_v4(), Utc::now())
    .await
    .expect("the session starts");

    assert_eq!(
        begun
            .cookie_changes
            .cookies
            .iter()
            .map(Cookie::name)
            .collect::<Vec<&str>>(),
        [
            "__Secure-margaret-session",
            "__Secure-margaret-session-access"
        ]
    );
    assert!(
        begun
            .cookie_changes
            .cookies
            .iter()
            .all(|cookie| cookie.domain() == Some("intentee.ai"))
    );
}
