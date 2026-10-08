use std::sync::Arc;

use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::forms::session_cookie::SessionCookie;
use crate::models::session_with_user::SessionWithUser;
use crate::models::user::User;
use crate::models::user_account::UserAccount;
use crate::models::user_session::UserSession;

#[singleton]
#[infers_authenticated_user(user_model = User)]
pub struct SessionUserProvider {
    database: Arc<Database>,
}

impl SessionUserProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub async fn infer_session_user(
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

            match UserSession::query()
                .id
                .eq(session)
                .load::<SessionWithUser, _>(self.database.as_ref())
                .await?
            {
                Lookup::Found(SessionWithUser {
                    session:
                        UserSession {
                            authenticated_at, ..
                        },
                    user: UserAccount { id, name },
                }) => AuthenticatedUserOutcome::Authenticated(User {
                    authenticated_at,
                    id,
                    name,
                }),
                Lookup::Missing => AuthenticatedUserOutcome::Anonymous,
            }
        })
    }
}
