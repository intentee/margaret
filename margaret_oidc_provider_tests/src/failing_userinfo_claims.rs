use async_trait::async_trait;
use serde_json::Value;

use margaret_oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret_oidc_provider::userinfo_claims::UserinfoClaims;
use margaret_oidc_provider::userinfo_grant::UserinfoGrant;

pub struct FailingUserinfoClaims;

#[async_trait]
impl ProvidesUserinfoClaims for FailingUserinfoClaims {
    type Claims = Value;

    async fn claims(&self, _grant: &UserinfoGrant) -> anyhow::Result<UserinfoClaims<Value>> {
        Err(anyhow::anyhow!("the profile store is unreachable"))
    }
}
