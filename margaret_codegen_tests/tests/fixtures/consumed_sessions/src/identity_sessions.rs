use margaret::framework::macros::consumes_sessions;

#[consumes_sessions(
    issuer = identity,
    cookie_domain_from = "FIXTURE_SESSION_COOKIE_DOMAIN",
    refresh_url_from = "FIXTURE_SESSION_REFRESH_URL"
)]
pub struct IdentitySessions;
