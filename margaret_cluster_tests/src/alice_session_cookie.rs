use margaret_cluster_fixture::alice_session::ALICE_SESSION;
use margaret_cluster_fixture::forms::session_cookie::SessionCookie;

#[must_use]
pub fn alice_session_cookie() -> String {
    SessionCookie::issued(ALICE_SESSION).stripped().to_string()
}
