use async_trait::async_trait;

use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: Request) -> Response;
}
