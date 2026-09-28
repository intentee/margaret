use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::forms::session_cookie::SessionCookie;
use crate::models::user::User;
use crate::stores::user_store::UserStore;

#[singleton]
#[infers_authenticated_user(user_model = User)]
pub struct SessionUserProvider {
    users: Arc<UserStore>,
}

impl SessionUserProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(users: Arc<UserStore>) -> anyhow::Result<Self> {
        Ok(Self { users })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_session_user(
        &self,
        #[form_request(from = RequestInput::Cookie)] cookie: SessionCookie,
    ) -> anyhow::Result<AuthenticatedUserOutcome<User>> {
        Ok({
            let Some(session) = cookie.session else {
                return Ok(AuthenticatedUserOutcome::Anonymous);
            };
            let Ok(session) = Uuid::parse_str(&session) else {
                return Ok(AuthenticatedUserOutcome::Interrupted(
                    ResponseContinuation::from(Response::text(400, "Malformed session cookie")),
                ));
            };

            match self.users.find_user_by_session(session) {
                Some(user) => AuthenticatedUserOutcome::Authenticated(user),
                None => AuthenticatedUserOutcome::Anonymous,
            }
        })
    }
}
