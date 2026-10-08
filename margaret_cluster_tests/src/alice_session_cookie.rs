use margaret_cluster_fixture::forms::session_cookie::SessionCookie;
use margaret_cluster_fixture::stores::alice_session::ALICE_SESSION;

#[must_use]
pub fn alice_session_cookie() -> String {
    SessionCookie::issued(ALICE_SESSION).stripped().to_string()
}
