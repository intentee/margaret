use std::sync::Arc;

use async_trait::async_trait;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_userinfo_claims;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret::framework::oidc_provider::userinfo_claims::UserinfoClaims;
use margaret::framework::oidc_provider::userinfo_grant::UserinfoGrant;

use crate::auth::profile_claims::ProfileClaims;
use crate::models::user_account::UserAccount;

#[singleton]
#[provides_userinfo_claims]
pub struct ProfileClaimsProvider {
    database: Arc<Database>,
}

impl ProfileClaimsProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(database: Arc<Database>) -> anyhow::Result<Self> {
        Ok(Self { database })
    }
}

#[async_trait]
impl ProvidesUserinfoClaims for ProfileClaimsProvider {
    type Claims = ProfileClaims;

    async fn claims(&self, grant: &UserinfoGrant) -> anyhow::Result<UserinfoClaims<ProfileClaims>> {
        Ok(
            match UserAccount::query()
                .id
                .eq(grant.subject)
                .find(self.database.as_ref())
                .await?
            {
                Lookup::Found(UserAccount { name, .. }) => {
                    UserinfoClaims::Found(ProfileClaims { name })
                }
                Lookup::Missing => UserinfoClaims::SubjectUnknown,
            },
        )
    }
}
