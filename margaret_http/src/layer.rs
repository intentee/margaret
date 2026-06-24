use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::response::Response;

pub fn layer<M>(middleware: Arc<M>, marker: M::Marker, inner: Arc<dyn Handler>) -> Arc<dyn Handler>
where
    M: HttpMiddleware + Send + Sync + 'static,
{
    Arc::new(LayeredHandler {
        inner,
        marker,
        middleware,
    })
}

struct LayeredHandler<M: HttpMiddleware> {
    inner: Arc<dyn Handler>,
    marker: M::Marker,
    middleware: Arc<M>,
}

#[async_trait]
impl<M> Handler for LayeredHandler<M>
where
    M: HttpMiddleware + Send + Sync + 'static,
{
    async fn handle(&self, request: Request) -> Response {
        self.middleware
            .process(request, self.marker.clone(), Next::new(self.inner.clone()))
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::layer;
    use crate::handler::Handler;
    use crate::http_middleware::HttpMiddleware;
    use crate::method::Method;
    use crate::next::Next;
    use crate::request::Request;
    use crate::response::Response;

    struct Inner;

    #[async_trait]
    impl Handler for Inner {
        async fn handle(&self, _request: Request) -> Response {
            Response::text(200, "inner")
        }
    }

    struct Gate;

    #[async_trait]
    impl HttpMiddleware for Gate {
        type Marker = u16;

        async fn process(&self, request: Request, marker: u16, next: Next) -> Response {
            if marker == 200 {
                next.run(request).await
            } else {
                Response::text(marker, "blocked")
            }
        }
    }

    async fn status_with_marker(marker: u16) -> u16 {
        layer(Arc::new(Gate), marker, Arc::new(Inner))
            .handle(Request::new(Method::Get, "/".to_string()))
            .await
            .into_http()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn runs_the_inner_handler_when_the_marker_permits() {
        assert_eq!(status_with_marker(200).await, 200);
    }

    #[tokio::test]
    async fn lets_the_middleware_short_circuit_on_its_marker() {
        assert_eq!(status_with_marker(403).await, 403);
    }
}
