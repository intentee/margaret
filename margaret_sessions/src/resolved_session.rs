use margaret_http::cookie_changes::CookieChanges;

use crate::session::Session;

pub struct ResolvedSession {
    pub cookie_changes: CookieChanges,
    pub session: Option<Session>,
}
