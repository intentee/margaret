use chrono::Utc;
use cookie::Cookie;

use margaret_sessions::issued_sessions::IssuedSessions;

use crate::end_user_subject::END_USER_SUBJECT;

/// # Panics
///
/// Panics when the session of the fixture end user cannot start.
pub async fn signed_in_session_cookies(sessions: &IssuedSessions) -> Vec<Cookie<'static>> {
    sessions
        .start(END_USER_SUBJECT, Utc::now())
        .await
        .expect("the session of the end user starts")
        .cookie_changes
        .cookies
}
