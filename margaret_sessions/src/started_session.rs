use margaret_http::cookie_changes::CookieChanges;

use crate::session::Session;

pub struct StartedSession {
    pub cookie_changes: CookieChanges,
    pub session: Session,
}
