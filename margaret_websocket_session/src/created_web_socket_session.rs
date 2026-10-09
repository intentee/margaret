use std::sync::Arc;

use margaret_http::cookie_changes::CookieChanges;

pub struct CreatedWebSocketSession<Session> {
    pub cookie_changes: CookieChanges,
    pub session: Arc<Session>,
}
