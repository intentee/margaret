use async_trait::async_trait;

use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait DeferredInterception: Send {
    async fn render(self: Box<Self>, request: &Request) -> Response;
}
