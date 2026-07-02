use async_trait::async_trait;

use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

#[async_trait]
pub trait DeferredInterception: Send {
    async fn render(self: Box<Self>, request: &Request) -> ResponseContinuation;
}
