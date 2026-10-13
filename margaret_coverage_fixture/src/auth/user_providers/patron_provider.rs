use margaret::framework::http::request::Request;
use margaret::framework::http::response::Response;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::auth::authenticated_users::patron::Patron;
use crate::auth::headers::patron_card::PATRON_CARD;

#[singleton]
#[infers_authenticated_user(user_model = Patron)]
pub struct PatronProvider;

impl PatronProvider {
    #[must_use]
    #[infer_from_request]
    pub fn infer_patron(&self, request: &Request) -> AuthenticatedUserOutcome<Patron> {
        let Some(card) = request.inputs.server.header(&PATRON_CARD) else {
            return AuthenticatedUserOutcome::Anonymous;
        };

        match card.parse() {
            Ok(card) => AuthenticatedUserOutcome::Authenticated(Patron { card }),
            Err(error) => {
                AuthenticatedUserOutcome::Interrupted(ResponseContinuation::from(Response::text(
                    400,
                    format!("the patron card is not a card number: {error}"),
                )))
            }
        }
    }
}
