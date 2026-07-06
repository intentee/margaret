use async_trait::async_trait;
use serde::Serialize;

use crate::jwks_key_error::JwksKeyError;

#[async_trait]
pub trait SignsClaims {
    async fn sign<TClaims: Send + Serialize + Sync>(
        &self,
        claims: &TClaims,
    ) -> Result<String, JwksKeyError>;
}
