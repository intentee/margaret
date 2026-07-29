use failures::Result as InferenceResult;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use super::user::User;

#[singleton]
#[infers_authenticated_user(user_model = crate::user::User)]
pub struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn infer(&self) -> InferenceResult<AuthenticatedUserOutcome<User>> {
        Ok(AuthenticatedUserOutcome::Anonymous)
    }
}
