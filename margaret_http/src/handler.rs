use async_trait::async_trait;

use crate::request::Request;
use crate::responded::Responded;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: Request) -> Responded;
}
