use async_trait::async_trait;

use crate::access_decision::AccessDecision;
use crate::request::Request;

#[async_trait]
pub trait AccessPolicy: Send + Sync {
    async fn decide(&self, request: &Request) -> anyhow::Result<AccessDecision>;
}
