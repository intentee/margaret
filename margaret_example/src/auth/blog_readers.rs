use std::sync::Arc;

use async_trait::async_trait;
use serde::de::IgnoredAny;
use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::admits_sign_in;
use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_sign_in::admits_sign_in::AdmitsSignIn;
use margaret::framework::oidc_sign_in::sign_in_admission::SignInAdmission;
use margaret::framework::oidc_sign_in::sign_in_flow::SignInFlow;
use margaret::framework::oidc_sign_in::signed_in::SignedIn;

use crate::models::user_account::UserAccount;

#[singleton]
#[admits_sign_in(client = blog)]
pub struct BlogReaders {
    database: Arc<Database>,
}

impl BlogReaders {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
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

        Ok(
            match UserAccount::query()
                .id
                .eq(reader)
                .find(self.database.as_ref())
                .await?
            {
                Lookup::Found(UserAccount { id, .. }) => SignInAdmission::Admitted(id),
                Lookup::Missing => SignInAdmission::Refused,
            },
        )
    }
}
