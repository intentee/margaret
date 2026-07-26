use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::identity::authenticated_user_inference::AuthenticatedUserInference;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::forms::session_cookie::SessionCookie;
use crate::models::user::User;
use crate::stores::user_store::UserStore;

#[singleton]
#[infers_authenticated_user(
    login_route = crate::routes::public::get_sign_in::GetSignIn,
    user_model = crate::models::user::User
)]
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
    ) -> anyhow::Result<AuthenticatedUserInference<User>> {
        let Some(session) = cookie.session else {
            return Ok(AuthenticatedUserInference::Anonymous);
        };
        let Ok(session) = Uuid::parse_str(&session) else {
            return Ok(AuthenticatedUserInference::LoginRequired);
        };

        Ok(match self.users.find_user_by_session(session) {
            Some(user) => AuthenticatedUserInference::Authenticated(user),
            None => AuthenticatedUserInference::Anonymous,
        })
    }
}
