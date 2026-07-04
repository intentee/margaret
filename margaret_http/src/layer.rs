use std::sync::Arc;

use async_trait::async_trait;

use crate::handler::Handler;
use crate::http_middleware::HttpMiddleware;
use crate::next::Next;
use crate::request::Request;
use crate::response_continuation::ResponseContinuation;

struct LayeredHandler<Middleware: HttpMiddleware> {
    inner: Arc<dyn Handler>,
    middleware: Arc<Middleware>,
}

#[async_trait]
impl<Middleware> Handler for LayeredHandler<Middleware>
where
    Middleware: HttpMiddleware + Send + Sync + 'static,
{
    async fn handle(&self, request: &Request) -> ResponseContinuation {
        self.middleware
            .process(request, Next::new(self.inner.clone()))
            .await
    }
}

pub fn layer<Middleware>(middleware: Arc<Middleware>, inner: Arc<dyn Handler>) -> Arc<dyn Handler>
where
    Middleware: HttpMiddleware + Send + Sync + 'static,
{
    Arc::new(LayeredHandler { inner, middleware })
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
    use crate::respond_recursively::respond_recursively;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;
    use crate::servers::Servers;

    struct Inner;

    #[async_trait]
    impl Handler for Inner {
        async fn handle(&self, _request: &Request) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(200, "inner"))
        }
    }

    struct PassThrough;

    #[async_trait]
    impl HttpMiddleware for PassThrough {
        async fn process(&self, request: &Request, next: Next) -> ResponseContinuation {
            next.run(request).await
        }
    }

    struct ShortCircuit;

    #[async_trait]
    impl HttpMiddleware for ShortCircuit {
        async fn process(&self, _request: &Request, _next: Next) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(403, "blocked"))
        }
    }

    async fn status_through<Middleware>(middleware: Middleware) -> u16
    where
        Middleware: HttpMiddleware + Send + Sync + 'static,
    {
        let servers = Arc::new(Servers::new(Vec::new(), Vec::new()));

        respond_recursively(
            &servers,
            Request::new(Method::Get, "/".to_string()),
            layer(Arc::new(middleware), Arc::new(Inner)),
        )
        .await
        .into_http()
        .status()
        .as_u16()
    }

    #[tokio::test]
    async fn runs_the_inner_handler_when_the_middleware_delegates() {
        assert_eq!(status_through(PassThrough).await, 200);
    }

    #[tokio::test]
    async fn lets_the_middleware_short_circuit_without_the_inner_handler() {
        assert_eq!(status_through(ShortCircuit).await, 403);
    }
}
