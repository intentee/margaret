use std::sync::Arc;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;
use margaret::framework::macros::constructor;
use margaret::framework::macros::infer_from_request;
use margaret::framework::macros::infers_authenticated_user;
use margaret::framework::macros::singleton;
use margaret::framework::sessions::session::Session;

use crate::auth::authenticated_users::user::User;
use crate::models::user_account::UserAccount;
use crate::repositories::user_account_repository::UserAccountRepository;

#[singleton]
#[infers_authenticated_user(user_model = User)]
pub struct SessionUserProvider {
    user_accounts: Arc<UserAccountRepository>,
}

impl SessionUserProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(user_accounts: Arc<UserAccountRepository>) -> anyhow::Result<Self> {
        Ok(Self { user_accounts })
    }

    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[infer_from_request]
    pub async fn infer_session_user(
        &self,
        #[session(issuer = provider)] session: Option<Session>,
    ) -> anyhow::Result<AuthenticatedUserOutcome<User>> {
        let Some(Session {
            authenticated_at,
            subject,
            ..
        }) = session
        else {
            return Ok(AuthenticatedUserOutcome::Anonymous);
        };

        Ok(match self.user_accounts.find(subject).await? {
            Lookup::Found(UserAccount { id, name }) => {
                AuthenticatedUserOutcome::Authenticated(User {
                    authenticated_at,
                    id,
                    name,
                })
            }
            Lookup::Missing => AuthenticatedUserOutcome::Anonymous,
        })
    }
}
