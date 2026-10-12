use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Map;
use serde_json::Value;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::macros::constructor;
use margaret::framework::macros::provides_userinfo_claims;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret::framework::oidc_provider::userinfo_claims::UserinfoClaims;
use margaret::framework::oidc_provider::userinfo_grant::UserinfoGrant;

use crate::auth::scopes::profile_scope::ProfileScope;
use crate::models::user_account::UserAccount;
use crate::repositories::user_account_repository::UserAccountRepository;

#[singleton]
#[provides_userinfo_claims]
pub struct ProfileClaimsProvider {
    user_accounts: Arc<UserAccountRepository>,
}

impl ProfileClaimsProvider {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[constructor]
    pub fn create(user_accounts: Arc<UserAccountRepository>) -> anyhow::Result<Self> {
        Ok(Self { user_accounts })
    }
}

#[async_trait]
impl ProvidesUserinfoClaims for ProfileClaimsProvider {
    type Claims = Map<String, Value>;

    async fn claims(
        &self,
        grant: &UserinfoGrant,
    ) -> anyhow::Result<UserinfoClaims<Map<String, Value>>> {
        if !grant.includes::<ProfileScope>() {
            return Ok(UserinfoClaims::Found(Map::new()));
        }

        Ok(match self.user_accounts.find(grant.subject).await? {
            Lookup::Found(UserAccount { name, .. }) => {
                UserinfoClaims::Found(Map::from_iter([("name".to_string(), Value::String(name))]))
            }
            Lookup::Missing => UserinfoClaims::SubjectUnknown,
        })
    }
}
