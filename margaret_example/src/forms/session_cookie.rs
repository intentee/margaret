use cookie::Cookie;
use cookie::SameSite;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SessionCookie {
    pub session: Option<String>,
}

impl SessionCookie {
    #[must_use]
    pub fn issued(session: Uuid) -> Cookie<'static> {
        Cookie::build(("session", session.to_string()))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Lax)
            .secure(true)
            .build()
    }
}
