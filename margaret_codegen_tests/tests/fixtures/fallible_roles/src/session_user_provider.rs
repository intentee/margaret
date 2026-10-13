use failures::Result as InferenceResult;

use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use super::user::User;

#[singleton]
#[infers_authenticated_user(user_model = User)]
pub struct SessionUserProvider;

impl SessionUserProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer(&self, request: &Request) -> InferenceResult<AuthenticatedUserOutcome<User>> {
        Ok(match request.inputs.query.get("user") {
            Some(name) => AuthenticatedUserOutcome::Authenticated(User { name: name.clone() }),
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
