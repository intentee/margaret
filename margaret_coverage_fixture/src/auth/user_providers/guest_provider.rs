use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::guest::Guest;

#[singleton]
#[infers_authenticated_user(user_model = Guest)]
pub struct GuestProvider;

impl GuestProvider {
    #[must_use]
    #[infer_from_request]
    pub fn infer_guest(&self, request: &Request) -> AuthenticatedUserPresence<Guest> {
        AuthenticatedUserPresence::Authenticated(Guest {
            address: request.inputs.server.remote_addr(),
        })
    }
}
