use async_trait::async_trait;

use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait HttpResponder: Send + Sync {
    async fn respond(&self, request: Request) -> Response;
}
