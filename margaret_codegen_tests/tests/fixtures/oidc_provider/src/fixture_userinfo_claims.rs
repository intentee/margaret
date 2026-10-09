use async_trait::async_trait;
use serde_json::Map;
use serde_json::Value;

use margaret::framework::macros::provides_userinfo_claims;
use margaret::framework::macros::singleton;
use margaret::framework::oidc_provider::provides_userinfo_claims::ProvidesUserinfoClaims;
use margaret::framework::oidc_provider::userinfo_claims::UserinfoClaims;
use margaret::framework::oidc_provider::userinfo_grant::UserinfoGrant;

#[singleton]
#[provides_userinfo_claims]
pub struct FixtureUserinfoClaims;

#[async_trait]
impl ProvidesUserinfoClaims for FixtureUserinfoClaims {
    type Claims = Map<String, Value>;

    async fn claims(
        &self,
        _grant: &UserinfoGrant,
    ) -> anyhow::Result<UserinfoClaims<Map<String, Value>>> {
        Ok(UserinfoClaims::Found(Map::new()))
    }
}
