use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::session_cookie::SessionCookie;
use crate::models::user::User;
use crate::stores::user_store::UserStore;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_sign_out",
    path = "/sign-out",
    server = "public"
)]
pub struct PostSignOut {
    users: Arc<UserStore>,
}

impl PostSignOut {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(users: Arc<UserStore>) -> anyhow::Result<Self> {
        Ok(Self { users })
    }

    /// # Errors
    ///
    /// Returns an error when the session cannot be ended.
    #[process]
    pub async fn respond(
        &self,
        #[authenticated_user] _user: User,
        #[form_request(from = RequestInput::Cookie)] SessionCookie { session }: SessionCookie,
    ) -> anyhow::Result<Response> {
        Ok(match session.as_deref().map(Uuid::parse_str) {
            Some(Ok(session)) => {
                self.users.end_session(session).await?;

                Response::text(200, "signed out")
            }
            Some(Err(_)) | None => Response::unauthorized(),
        })
    }
}
