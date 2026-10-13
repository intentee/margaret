use std::sync::Arc;

use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::courier::Courier;
use crate::constants::COURIER_NAME;
use crate::singletons::roster::Roster;

#[singleton]
#[infers_authenticated_user(user_model = Courier)]
pub struct CourierProvider {
    roster: Arc<Roster>,
}

impl CourierProvider {
    #[must_use]
    #[constructor]
    pub fn create(roster: Arc<Roster>) -> Self {
        Self { roster }
    }

    /// # Errors
    ///
    /// Returns an error when the roster cannot be read.
    #[infer_from_request]
    pub async fn infer_courier(
        &self,
        request: &Request,
    ) -> anyhow::Result<AuthenticatedUserPresence<Courier>> {
        let Some(name) = request.inputs.server.header(&COURIER_NAME) else {
            return Ok(AuthenticatedUserPresence::Anonymous);
        };

        Ok(if self.roster.lists(name).await? {
            AuthenticatedUserPresence::Authenticated(Courier {
                name: name.to_string(),
            })
        } else {
            AuthenticatedUserPresence::Anonymous
        })
    }
}
