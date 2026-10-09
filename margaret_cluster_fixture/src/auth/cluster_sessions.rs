use margaret::framework::macros::issues_sessions;
use margaret::framework::sessions::session_cookies::SessionCookies;

#[issues_sessions(
    issuer = provider,
    audience = "margaret-cluster",
    cookies = SessionCookies::HostOnly
)]
pub struct ClusterSessions;
