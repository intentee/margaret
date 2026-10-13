use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::librarian::Librarian;
use crate::constants::LIBRARIAN_BADGE;
use crate::singletons::roster::Roster;

#[singleton]
#[infers_authenticated_user(user_model = Librarian)]
pub struct LibrarianProvider {
    roster: Arc<Roster>,
}

impl LibrarianProvider {
    #[must_use]
    #[constructor]
    pub fn create(roster: Arc<Roster>) -> Self {
        Self { roster }
    }

    /// # Errors
    ///
    /// Returns an error when the roster cannot be read.
    #[infer_from_request]
    pub async fn infer_librarian(
        &self,
        request: &Request,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Librarian>> {
        let Some(badge) = request.inputs.server.header(&LIBRARIAN_BADGE) else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        Ok(if self.roster.lists(badge).await? {
            AuthenticatedUserOutcome::Authenticated(Librarian {
                badge: badge.to_string(),
            })
        } else {
            AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::forbidden()))
        })
    }
}
