use std::sync::Arc;

use async_trait::async_trait;

use margaret_cookie_jar::cookie_jar::CookieJar;

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
    async fn handle(&self, request: &Request, cookie_jar: &CookieJar) -> ResponseContinuation {
        self.middleware
            .process(
                request,
                cookie_jar,
                Next::new(self.inner.clone(), cookie_jar),
            )
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
    use http::HeaderMap;
    use http::Method;

    use margaret_cookie_jar::cookie_jar::CookieJar;

    use super::layer;
    use crate::forward_targets::ForwardTargets;
    use crate::handler::Handler;
    use crate::http_middleware::HttpMiddleware;
    use crate::next::Next;
    use crate::request::Request;
    use crate::respond_recursively::respond_recursively;
    use crate::response::Response;
    use crate::response_continuation::ResponseContinuation;

    struct Inner;

    #[async_trait]
    impl Handler for Inner {
        async fn handle(
            &self,
            _request: &Request,
            _cookie_jar: &CookieJar,
        ) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(200, "inner"))
        }
    }

    struct PassThrough;

    #[async_trait]
    impl HttpMiddleware for PassThrough {
        async fn process<'jar>(
            &self,
            request: &Request,
            _cookie_jar: &'jar CookieJar,
            next: Next<'jar>,
        ) -> ResponseContinuation {
            next.run(request).await
        }
    }

    struct ShortCircuit;

    #[async_trait]
    impl HttpMiddleware for ShortCircuit {
        async fn process<'jar>(
            &self,
            _request: &Request,
            _cookie_jar: &'jar CookieJar,
            _next: Next<'jar>,
        ) -> ResponseContinuation {
            ResponseContinuation::Done(Response::text(403, "blocked"))
        }
    }

    async fn status_through<Middleware>(middleware: Middleware) -> u16
    where
        Middleware: HttpMiddleware + Send + Sync + 'static,
    {
        let forward_targets = Arc::new(ForwardTargets::new(Vec::new()));

        respond_recursively(
            &forward_targets,
            Request::new(Method::GET, "/".to_string()),
            &CookieJar::from_headers(&HeaderMap::new()).expect("an empty cookie jar is built"),
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
