use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::removal::Removal;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::forms::session_cookie::SessionCookie;
use crate::models::user::User;
use crate::models::user_session::UserSession;

#[singleton]
#[responds_to_http(
    method = RouteMethod::Post,
    name = "post_sign_out",
    path = "/sign-out",
    server = "public"
)]
pub struct PostSignOut {
    database: Arc<Database>,
}

impl PostSignOut {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
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
            Some(Ok(session)) => match UserSession::query()
                .id
                .eq(session)
                .delete(self.database.as_ref())
                .await?
            {
                Removal::Removed(_) | Removal::Missing => Response::text(200, "signed out"),
            },
            Some(Err(_)) | None => Response::unauthorized(),
        })
    }
}
