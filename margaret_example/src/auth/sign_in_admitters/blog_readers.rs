use std::sync::Arc;

use async_trait::async_trait;
use serde::de::IgnoredAny;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::macros::admits_sign_in;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::admits_sign_in::AdmitsSignIn;
use margaret::framework::oidc_sign_in::sign_in_admission::SignInAdmission;
use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;
use margaret::framework::oidc_sign_in::signed_in::SignedIn;

use crate::models::user_account::UserAccount;
use crate::repositories::user_account_repository::UserAccountRepository;

#[singleton]
#[admits_sign_in(client = blog)]
pub struct BlogReaders {
    user_accounts: Arc<UserAccountRepository>,
}

impl BlogReaders {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(user_accounts: Arc<UserAccountRepository>) -> anyhow::Result<Self> {
        Ok(Self { user_accounts })
    }
}

#[async_trait]
impl AdmitsSignIn for BlogReaders {
    type IdClaims = IgnoredAny;

    async fn admit(
        &self,
        SignedIn { subject, .. }: SignedIn<IgnoredAny>,
        _flow: &SignInFlow,
    ) -> anyhow::Result<SignInAdmission> {
        let Ok(reader) = Uuid::try_parse(&subject) else {
            return Ok(SignInAdmission::Refused);
        };

        Ok(match self.user_accounts.find(reader).await? {
            Lookup::Found(UserAccount { id, .. }) => SignInAdmission::Admitted(id),
            Lookup::Missing => SignInAdmission::Refused,
        })
    }
}
