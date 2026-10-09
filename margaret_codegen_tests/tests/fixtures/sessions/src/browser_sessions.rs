use margaret::framework::macros::issues_sessions;
use margaret::framework::sessions::session_cookies::SessionCookies;

#[issues_sessions(
    issuer = provider,
    audience = "browser",
    cookies = SessionCookies::SharedWithDomain(domain_from = "FIXTURE_SESSION_COOKIE_DOMAIN")
)]
pub struct BrowserSessions;
