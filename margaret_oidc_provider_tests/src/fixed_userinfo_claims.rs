use async_trait::async_trait;
use serde::Serialize;

use margaret_oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret_oidc_provider::userinfo_claims::UserinfoClaims;
use margaret_oidc_provider::userinfo_grant::UserinfoGrant;

pub struct FixedUserinfoClaims<TClaims> {
    pub claims: TClaims,
}

#[async_trait]
impl<TClaims> ProvidesUserinfoClaims for FixedUserinfoClaims<TClaims>
where
    TClaims: Clone + Serialize + Send + Sync + 'static,
{
    type Claims = TClaims;

    async fn claims(&self, _grant: &UserinfoGrant) -> anyhow::Result<UserinfoClaims<TClaims>> {
        Ok(UserinfoClaims::Found(self.claims.clone()))
    }
}
