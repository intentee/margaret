use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::sessions::session::Session;

use crate::reader::Reader;

#[singleton]
#[infers_authenticated_user(user_model = Reader)]
pub struct ReaderProvider;

impl ReaderProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub fn infer_reader(
        &self,
        #[session(issuer = identity)] session: Option<Session>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<Reader>> {
        Ok(match session {
            Some(Session { subject, .. }) => {
                AuthenticatedUserOutcome::Authenticated(Reader { subject })
            }
            None => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
