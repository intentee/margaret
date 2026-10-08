use cookie::Cookie;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use margaret::framework::http::host_cookie::host_cookie;

#[derive(Deserialize, Validate)]
pub struct SessionCookie {
    pub session: Option<String>,
}

impl SessionCookie {
    #[must_use]
    pub fn issued(session: Uuid) -> Cookie<'static> {
        host_cookie("session".to_string(), session.to_string()).build()
    }
}
