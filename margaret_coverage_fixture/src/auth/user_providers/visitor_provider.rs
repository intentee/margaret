use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_presence::AuthenticatedUserPresence;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::visitor::Visitor;
use crate::constants::VISITOR_NAME;

#[singleton]
#[infers_authenticated_user(user_model = Visitor)]
pub struct VisitorProvider;

impl VisitorProvider {
    #[must_use]
    #[infer_from_request]
    pub fn infer_visitor(&self, request: &Request) -> AuthenticatedUserPresence<Visitor> {
        match request.inputs.server.header(&VISITOR_NAME) {
            Some(name) => AuthenticatedUserPresence::Authenticated(Visitor {
                name: name.to_string(),
            }),
            None => AuthenticatedUserPresence::Anonymous,
        }
    }
}
