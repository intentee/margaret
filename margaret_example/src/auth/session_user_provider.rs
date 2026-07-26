use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::forms::session_cookie::SessionCookie;
use crate::margaret::routes::Routes;
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
        routes: &Routes,
        #[form_request(from = Cookie)] cookie: SessionCookie,
    ) -> anyhow::Result<AuthenticatedUserOutcome<User>> {
        let Some(session) = cookie.session else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };
        let Ok(session) = Uuid::parse_str(&session) else {
            return Ok(AuthenticatedUserOutcome::LoginPageRedirect(
                routes.public.get_sign_in.see_other(),
            ));
        };

        Ok(match self.users.find_user_by_session(session) {
            Some(user) => AuthenticatedUserOutcome::Authenticated(user),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
