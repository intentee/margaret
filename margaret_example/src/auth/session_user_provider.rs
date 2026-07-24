use std::sync::Arc;

use uuid::Uuid;

use margaret_http::response::Response;
use margaret_http::response_continuation::ResponseContinuation;
use margaret_identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret_macros::constructor;
use margaret_macros::infer_from_request;
use margaret_macros::infers_authenticated_user;
use margaret_macros::singleton;

use crate::forms::session_cookie::SessionCookie;
use crate::models::user::User;
use crate::stores::user_store::UserStore;

#[singleton]
#[infers_authenticated_user(user_model = crate::models::user::User)]
pub struct SessionUserProvider {
    users: Arc<UserStore>,
}

impl SessionUserProvider {
    #[constructor]
    #[must_use]
    pub fn create(users: Arc<UserStore>) -> Self {
        Self { users }
    }

    #[infer_from_request]
    pub async fn infer_session_user(
        &self,
        #[form_request(from = Cookie)] cookie: SessionCookie,
    ) -> AuthenticatedUserOutcome<User> {
        let Some(session) = cookie.session else {
            return AuthenticatedUserOutcome::Anonymous;
        };
        let Ok(session) = Uuid::parse_str(&session) else {
            return AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(
                Response::text(400, "Malformed session cookie"),
            ));
        };

        match self.users.find_user_by_session(session) {
            Some(user) => AuthenticatedUserOutcome::Authenticated(user),
            None => AuthenticatedUserOutcome::Anonymous,
        }
    }
}
