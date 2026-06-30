use std::collections::HashMap;
use std::sync::Mutex;

use margaret_http::cookie::Cookie;
use margaret_http::cookie::SameSite;
use margaret_http::cookie::time::Duration;
use margaret_http::request::Request;
use margaret_http::response::Response;
use uuid::Uuid;

pub struct SessionAuthentication {
    cookie_name: String,
    sessions: Mutex<HashMap<String, String>>,
}

impl SessionAuthentication {
    pub fn new(cookie_name: impl Into<String>) -> Self {
        Self {
            cookie_name: cookie_name.into(),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_authenticated_actor(
        &self,
        request: &Request,
        response: Response,
        user_id: &str,
    ) -> Response {
        let session_id = match request.cookie(&self.cookie_name) {
            Some(existing) => existing,
            None => Uuid::new_v4().to_string(),
        };

        self.sessions
            .lock()
            .expect("the session store lock is not poisoned")
            .insert(session_id.clone(), user_id.to_string());

        response.set_cookie(self.active_cookie(session_id))
    }

    pub fn forget(&self, request: &Request, response: Response) -> Response {
        if let Some(session_id) = request.cookie(&self.cookie_name) {
            self.sessions
                .lock()
                .expect("the session store lock is not poisoned")
                .remove(&session_id);
        }

        response.set_cookie(self.expired_cookie())
    }

    pub fn authenticated_actor_id(&self, request: &Request) -> Option<String> {
        let session_id = request.cookie(&self.cookie_name)?;

        self.sessions
            .lock()
            .expect("the session store lock is not poisoned")
            .get(&session_id)
            .cloned()
    }

    fn active_cookie(&self, session_id: String) -> Cookie<'static> {
        Cookie::build((self.cookie_name.clone(), session_id))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Lax)
            .build()
    }

    fn expired_cookie(&self) -> Cookie<'static> {
        Cookie::build((self.cookie_name.clone(), String::new()))
            .http_only(true)
            .path("/")
            .same_site(SameSite::Lax)
            .max_age(Duration::ZERO)
            .build()
    }
}
