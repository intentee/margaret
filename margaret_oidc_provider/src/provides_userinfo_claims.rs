use async_trait::async_trait;
use serde::Serialize;

use crate::userinfo_claims::UserinfoClaims;
use crate::userinfo_grant::UserinfoGrant;

#[async_trait]
pub trait ProvidesUserinfoClaims: Send + Sync + 'static {
    type Claims: Serialize + Send;

    async fn claims(&self, grant: &UserinfoGrant) -> anyhow::Result<UserinfoClaims<Self::Claims>>;
}
