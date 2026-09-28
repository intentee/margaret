use tokio::task;

use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;

use crate::collision::reader::Reader;

#[singleton]
#[infers_authenticated_user(user_model = Reader)]
pub struct Session;

impl Session {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create() -> anyhow::Result<Self> {
        Ok(Self)
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub async fn infer_reader(&self) -> anyhow::Result<AuthenticatedUserOutcome<Reader>> {
        task::yield_now().await;

        Ok(AuthenticatedUserOutcome::Authenticated(Reader {
            name: "milo".to_string(),
        }))
    }
}
