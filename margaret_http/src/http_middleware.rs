use async_trait::async_trait;

use crate::next::Next;
use crate::request::Request;
use crate::response::Response;

#[async_trait]
pub trait HttpMiddleware: Send + Sync {
    type Marker: Clone + Send + Sync + 'static;

    async fn process(&self, request: Request, marker: Self::Marker, next: Next) -> Response;
}
