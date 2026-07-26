use async_trait::async_trait;

use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait HttpMiddleware: Send + Sync {
    async fn process(&self, request: &Request, next: Next) -> anyhow::Result<ResponseContinuation>;
}
